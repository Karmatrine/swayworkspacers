# swayworkspacers
.json output of workspaces on a particular screen using rust swayipc crate

Useful for monitor-aware widget creation in the eww bar.

Example of simple widget:

```
(deflisten workspace0 "~/.config/eww/scripts/swayworkspacers DP-1")
(defwidget workspaces0 []
  (eventbox
    (box :class "workspaces"
      :space-evenly false
      (for i in workspace0
        (button
          :onclick "sway workspace ${i.id}"
          :class "${i.class}"
          "${i.name}"
        )
      )
    )
  )
)
```
