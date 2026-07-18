//! App-level and window-level gpui actions for FullTime.

use gpui::actions;

// App-level actions
actions!(fulltime,
         [Quit, HideApplication, HideOthers, ShowAll, About]);

// Window-level actions
actions!(fulltime, [Minimize, Zoom, ToggleFullscreen]);
