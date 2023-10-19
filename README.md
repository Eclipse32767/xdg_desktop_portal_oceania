# xdg_desktop_portal_oceania
A custom xdg portal planned to utilize narwhal for opening files, meant as a replacement for the gtk portal


# portals to absolutely implement

org.freedesktop.impl.portal.Access — Interface for presenting an access dialog
org.freedesktop.impl.portal.Account — Backend for the portal for obtaining user information
org.freedesktop.impl.portal.AppChooser — Interface for choosing an application
org.freedesktop.impl.portal.FileChooser — File chooser portal backend interface
org.freedesktop.impl.portal.Request — Shared request interface
org.freedesktop.impl.portal.Session — Shared session interface
org.freedesktop.impl.portal.Settings — Settings portal backend interface
org.freedesktop.impl.portal.Wallpaper — Portal for setting the desktop's Wallpaper

# portals to maybe implement

org.freedesktop.impl.portal.Clipboard
org.freedesktop.impl.portal.Background — Background portal backend interface
org.freedesktop.impl.portal.Email — Email portal backend interface
org.freedesktop.impl.portal.Inhibit — Inhibit portal backend interface
org.freedesktop.impl.portal.Lockdown — Lockdown backend interface
org.freedesktop.impl.portal.Notification — Notification portal backend interface
org.freedesktop.impl.portal.PermissionStore — Database to store permissions
org.freedesktop.impl.portal.Print — Print portal backend interface
org.freedesktop.impl.portal.Secret — Secret portal backend interface

# portals to absolutely not implement, screw you

org.freedesktop.impl.portal.ScreenCast — Screen cast portal backend interface
org.freedesktop.impl.portal.Screenshot — Screenshot portal backend interface
org.freedesktop.impl.portal.GlobalShortcuts
org.freedesktop.impl.portal.RemoteDesktop — Remote desktop portal backend interface
org.freedesktop.impl.portal.InputCapture — Capture input portal backend interface