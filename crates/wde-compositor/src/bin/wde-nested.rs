//! WDE nested development compositor.
//!
//! This initial backend follows the Smithay 0.7 minimal compositor pattern.
//! It is a protocol/rendering bring-up target, not the final WDE shell: window
//! decorations, panel rendering, robust input routing, and production DRM/TTY
//! support are separate milestones.

use std::{collections::BTreeMap, os::fd::OwnedFd, sync::Arc, time::Instant};

use ::winit::platform::pump_events::PumpStatus;
use smithay::{
    backend::{
        input::{InputEvent, KeyboardKeyEvent},
        renderer::{
            Color32F, Frame, Renderer,
            element::{
                Kind,
                surface::{WaylandSurfaceRenderElement, render_elements_from_surface_tree},
            },
            gles::GlesRenderer,
            utils::{draw_render_elements, on_commit_buffer_handler},
        },
        winit::{self, WinitEvent},
    },
    delegate_compositor, delegate_data_device, delegate_seat, delegate_shm, delegate_xdg_shell,
    input::{Seat, SeatHandler, SeatState, keyboard::FilterResult},
    reexports::wayland_server::{Display, protocol::wl_seat},
    utils::{Rectangle, Serial, Transform},
    wayland::{
        buffer::BufferHandler,
        compositor::{
            with_states, CompositorClientState, CompositorHandler, CompositorState,
            SurfaceAttributes, TraversalAction, with_surface_tree_downward,
        },
        selection::{
            SelectionHandler,
            data_device::{
                ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
            },
        },
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
            XdgToplevelSurfaceData,
        },
        shm::{ShmHandler, ShmState},
    },
};
use wayland_protocols::xdg::shell::server::xdg_toplevel;
use wayland_server::{
    Client, ListeningSocket,
    backend::{ClientData, ClientId, DisconnectReason},
    protocol::{
        wl_buffer,
        wl_surface::{self, WlSurface},
    },
};
use wde_compositor::{WindowAction, WindowId, WindowManager, WindowState};

struct App {
    compositor_state: CompositorState,
    xdg_shell_state: XdgShellState,
    shm_state: ShmState,
    seat_state: SeatState<Self>,
    data_device_state: DataDeviceState,
    seat: Seat<Self>,
    window_manager: WindowManager,
    window_surfaces: BTreeMap<WindowId, ToplevelSurface>,
}

#[derive(Default)]
struct ClientState {
    compositor_state: CompositorClientState,
}

impl App {
    fn window_id_for_surface(&self, surface: &ToplevelSurface) -> Option<WindowId> {
        self.window_surfaces
            .iter()
            .find_map(|(id, managed)| (managed == surface).then_some(*id))
    }

    fn sync_toplevel_metadata(&mut self, surface: &ToplevelSurface) {
        let Some(id) = self.window_id_for_surface(surface) else {
            return;
        };
        let metadata = with_states(surface.wl_surface(), |states| {
            let data = states.data_map.get::<XdgToplevelSurfaceData>()?;
            let attributes = data.lock().ok()?;
            Some((attributes.title.clone(), attributes.app_id.clone()))
        });
        if let Some((title, app_id)) = metadata {
            let _ = self.window_manager.update_metadata(id, title, app_id);
        }
    }
}

impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {}
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}

impl BufferHandler for App {
    fn buffer_destroyed(&mut self, _buffer: &wl_buffer::WlBuffer) {}
}

impl XdgShellHandler for App {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        if let Ok(id) = self.window_manager.register("Window") {
            self.window_surfaces.insert(id, surface.clone());
            self.sync_toplevel_metadata(&surface);
        }

        surface.with_pending_state(|state| {
            state.states.set(xdg_toplevel::State::Activated);
        });
        surface.send_configure();
    }

    fn maximize_request(&mut self, surface: ToplevelSurface) {
        if let Some(id) = self.window_id_for_surface(&surface) {
            if self.window_manager.get(id).is_some_and(|window| window.state == WindowState::Normal) {
                let _ = self.window_manager.apply(id, WindowAction::MaximizeOrRestore);
            }
        }
        surface.with_pending_state(|state| {
            state.states.set(xdg_toplevel::State::Maximized);
        });
        surface.send_configure();
    }

    fn unmaximize_request(&mut self, surface: ToplevelSurface) {
        if let Some(id) = self.window_id_for_surface(&surface) {
            if self.window_manager.get(id).is_some_and(|window| window.state == WindowState::Maximized) {
                let _ = self.window_manager.apply(id, WindowAction::MaximizeOrRestore);
            }
        }
        surface.with_pending_state(|state| {
            state.states.unset(xdg_toplevel::State::Maximized);
        });
        surface.send_configure();
    }

    fn minimize_request(&mut self, surface: ToplevelSurface) {
        if let Some(id) = self.window_id_for_surface(&surface) {
            let _ = self.window_manager.apply(id, WindowAction::Minimize);
        }
    }

    fn app_id_changed(&mut self, surface: ToplevelSurface) {
        self.sync_toplevel_metadata(&surface);
    }

    fn title_changed(&mut self, surface: ToplevelSurface) {
        self.sync_toplevel_metadata(&surface);
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {}
    fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {}
    fn reposition_request(
        &mut self,
        _surface: PopupSurface,
        _positioner: PositionerState,
        _token: u32,
    ) {
    }

    fn toplevel_destroyed(&mut self, surface: ToplevelSurface) {
        let id = self
            .window_surfaces
            .iter()
            .find_map(|(id, managed)| (managed == &surface).then_some(*id));

        if let Some(id) = id {
            self.window_surfaces.remove(&id);
            let _ = self.window_manager.unregister(id);
        }
    }
}

impl SelectionHandler for App {
    type SelectionUserData = ();
}

impl DataDeviceHandler for App {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }
}

impl ClientDndGrabHandler for App {}

impl ServerDndGrabHandler for App {
    fn send(&mut self, _mime_type: String, _fd: OwnedFd, _seat: Seat<Self>) {}
}

impl CompositorHandler for App {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<ClientState>().unwrap().compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler::<Self>(surface);
    }
}

impl ShmHandler for App {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

impl SeatHandler for App {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&WlSurface>) {}

    fn cursor_image(
        &mut self,
        _seat: &Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
    }
}

delegate_xdg_shell!(App);
delegate_compositor!(App);
delegate_shm!(App);
delegate_seat!(App);
delegate_data_device!(App);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }

    run_nested()
}

fn run_nested() -> Result<(), Box<dyn std::error::Error>> {
    let mut display: Display<App> = Display::new()?;
    let mut display_handle = display.handle();

    let compositor_state = CompositorState::new::<App>(&display_handle);
    let shm_state = ShmState::new::<App>(&display_handle, vec![]);
    let mut seat_state = SeatState::new();
    let seat = seat_state.new_wl_seat(&display_handle, "wde-seat");

    let mut state = App {
        compositor_state,
        xdg_shell_state: XdgShellState::new::<App>(&display_handle),
        shm_state,
        seat_state,
        data_device_state: DataDeviceState::new::<App>(&display_handle),
        seat,
        window_manager: WindowManager::new(),
        window_surfaces: BTreeMap::new(),
    };

    let socket_name =
        std::env::var("WDE_WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-wde".to_owned());
    let listener = ListeningSocket::bind(&socket_name)?;
    eprintln!("WDE nested compositor listening on {socket_name}");

    let mut clients: Vec<Client> = Vec::new();
    let (mut backend, mut event_loop) = winit::init::<GlesRenderer>()?;
    let start_time = Instant::now();
    let keyboard = state.seat.add_keyboard(Default::default(), 200, 200)?;

    loop {
        let status = event_loop.dispatch_new_events(|event| match event {
            WinitEvent::Resized { .. } => {}
            WinitEvent::Input(event) => match event {
                InputEvent::Keyboard { event } => {
                    keyboard.input::<(), _>(
                        &mut state,
                        event.key_code(),
                        event.state(),
                        0.into(),
                        0,
                        |_, _, _| FilterResult::Forward,
                    );
                }
                InputEvent::PointerMotionAbsolute { .. } => {
                    let focused_surface = state
                        .window_manager
                        .focused()
                        .and_then(|id| state.window_surfaces.get(&id))
                        .map(|surface| surface.wl_surface().clone());

                    if let Some(surface) = focused_surface {
                        keyboard.set_focus(&mut state, Some(surface), 0.into());
                    }
                }
                _ => {}
            },
            _ => {}
        });

        if let PumpStatus::Exit(_) = status {
            return Ok(());
        }

        let size = backend.window_size();
        let damage = Rectangle::from_size(size);
        {
            let (renderer, mut framebuffer) = backend.bind()?;
            let elements = state
                .window_surfaces
                .iter()
                .filter(|(id, _)| {
                    state.window_manager.get(**id).is_some_and(|window| {
                        window.state != WindowState::Minimized
                    })
                })
                .flat_map(|(_, surface)| {
                    render_elements_from_surface_tree(
                        renderer,
                        surface.wl_surface(),
                        (0, 0),
                        1.0,
                        1.0,
                        Kind::Unspecified,
                    )
                })
                .collect::<Vec<WaylandSurfaceRenderElement<GlesRenderer>>>();

            let mut frame = renderer.render(&mut framebuffer, size, Transform::Flipped180)?;
            frame.clear(Color32F::new(0.0, 0.0, 0.0, 1.0), &[damage])?;
            draw_render_elements(&mut frame, 1.0, &elements, &[damage])?;
            frame.finish()?;

            let frame_time = start_time.elapsed().as_millis() as u32;
            for surface in state.xdg_shell_state.toplevel_surfaces() {
                send_frame_callbacks(surface.wl_surface(), frame_time);
            }

            if let Some(stream) = listener.accept()? {
                let client =
                    display_handle.insert_client(stream, Arc::new(ClientState::default()))?;
                clients.push(client);
            }

            display.dispatch_clients(&mut state)?;
            display.flush_clients()?;
        }

        // Dispatch and flush Wayland clients before submitting the host frame.
        backend.submit(Some(&[damage]))?;
    }
}

fn send_frame_callbacks(surface: &wl_surface::WlSurface, time: u32) {
    with_surface_tree_downward(
        surface,
        (),
        |_, _, &()| TraversalAction::DoChildren(()),
        |_surface, states, &()| {
            for callback in states
                .cached_state
                .get::<SurfaceAttributes>()
                .current()
                .frame_callbacks
                .drain(..)
            {
                callback.done(time);
            }
        },
        |_, _, &()| true,
    );
}
