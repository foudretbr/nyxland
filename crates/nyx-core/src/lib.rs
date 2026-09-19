pub mod geometry;
pub mod layout;
pub mod workspace;

pub use geometry::Rect;
pub use layout::{calculate_tiled_rects, LayoutKind};
pub use workspace::{ManagedWindow, Workspace};
