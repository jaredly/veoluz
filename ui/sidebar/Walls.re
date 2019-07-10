module AddWall = {
  let size = 50;
  let color = "#555";
  let lw = "2px";
  let line =
    <svg width={string_of_int(size)} height={string_of_int(size)}>
      <path stroke=color strokeWidth=lw d="M10,10 L40,40" />
      <circle stroke=color strokeWidth=lw cx="10" cy="10" r="4" fill="white" />
      <circle stroke=color strokeWidth=lw cx="40" cy="40" r="4" fill="white" />
    </svg>;
  let arc =
    <svg width={string_of_int(size)} height={string_of_int(size)}>
      <path
        stroke=color
        strokeWidth=lw
        d="M15,15 A10 10 0 0 0 35 35"
        fill="none"
      />
      <circle stroke=color strokeWidth=lw cx="15" cy="15" r="4" fill="white" />
      <circle stroke=color strokeWidth=lw cx="35" cy="35" r="4" fill="white" />
    </svg>;
  let parabola =
    <svg width={string_of_int(size)} height={string_of_int(size)}>
      <path
        stroke=color
        strokeWidth=lw
        d="M10,10 C 15 50 35 50 40 10"
        fill="none"
      />
      <path
        stroke=color
        strokeWidth=lw
        strokeDasharray="2"
        d="M10 40 L40 40"
        fill="none"
      />
      <circle stroke=color strokeWidth=lw cx="10" cy="40" r="4" fill="white" />
      <circle stroke=color strokeWidth=lw cx="40" cy="40" r="4" fill="white" />
    </svg>;

  [@react.component]
  let make = (~ui, ~updateUi) => {
    let adding =
      switch (Js.nullToOption(ui##selection)) {
      | Some(obj) =>
        %js.deep
        obj["Adding"]
      | _ => None
      };
    <div className=Styles.row>
      <button
        onClick={_evt =>
          updateUi(
            [%js.deep
              ui["selection"].replace(
                adding == Some("Line")
                  ? Js.Null.empty
                  : Js.Null.return({
                      "Adding": Some("Line"),
                      "Multiple": None,
                      "Light": None,
                      "Wall": None,
                    }),
              )
            ],
          )
        }
        className={Styles.iconButton(adding == Some("Line"))}>
        line
      </button>
      {Styles.spacer(8)}
      <button
        onClick={_evt =>
          updateUi(
            [%js.deep
              ui["selection"].replace(
                adding == Some("Parabola")
                  ? Js.Null.empty
                  : Js.Null.return({
                      "Adding": Some("Parabola"),
                      "Multiple": None,
                      "Light": None,
                      "Wall": None,
                    }),
              )
            ],
          )
        }
        className={Styles.iconButton(adding == Some("Parabola"))}>
        parabola
      </button>
      {Styles.spacer(8)}
      <button
        onClick={_evt =>
          updateUi(
            [%js.deep
              ui["selection"].replace(
                adding == Some("Circle")
                  ? Js.Null.empty
                  : Js.Null.return({
                      "Adding": Some("Circle"),
                      "Multiple": None,
                      "Light": None,
                      "Wall": None,
                    }),
              )
            ],
          )
        }
        className={Styles.iconButton(adding == Some("Circle"))}>
        arc
      </button>
    </div>;
  };
};

[@react.component]
let make = (~config: Rust.config, ~ui: Rust.ui, ~update, ~updateUi) => {
  <div className=Styles.column>
    <div className=Styles.title> {React.string("Add Wall")} </div>
    {Styles.spacer(8)}
    <AddWall ui updateUi />
    {Styles.spacer(16)}
    <div className=Styles.title> {React.string("Walls")} </div>
    {Styles.spacer(8)}
    {config##walls
     ->Belt.Array.mapWithIndex((i, wall) =>
         <Wall
           selected={
             switch (ui##selection->Js.nullToOption) {
             | None => false
             | Some(selection) =>
               switch ([%js.deep selection["Wall"]]) {
               | None => false
               | Some((wid, _)) => i == wid
               }
             }
           }
           onSelect={() =>
             updateUi(
               [%js.deep
                 ui["selection"].replace(Js.Null.return(Rust.selectWall(i)))
               ],
             )
           }
           onDeselect={() =>
             updateUi([%js.deep ui["selection"].replace(Js.null)])
           }
           onHoverOut={() =>
             updateUi(
               [%js.deep
                 ui["hovered"].replace(Js.Null.empty)["mouse_over"].replace(
                   false,
                 )
               ],
             )
           }
           onHover={() =>
             updateUi(
               [%js.deep
                 ui["hovered"].replace(
                   Js.Null.return((
                     i,
                     {"Move": Some((0.0, 0.0)), "Handle": None},
                   )),
                 )["mouse_over"].
                   replace(
                   true,
                 )
               ],
             )
           }
           onRemove={() =>
             update(
               [%js.deep
                 config["walls"].map(walls => {
                   let walls = Js.Array.copy(walls);
                   Js.Array.removeCountInPlace(~pos=i, ~count=1, walls)
                   ->ignore;
                   walls;
                 })
               ],
               true,
             )
           }
           onChange={(wall, checkpoint) =>
             update(
               [%js.deep
                 config["walls"].map(walls => {
                   let walls = Js.Array.copy(walls);
                   walls[i] = wall;
                   walls;
                 })
               ],
               checkpoint,
             )
           }
           wall
           key={string_of_int(i)}
         />
       )
     ->React.array}
  </div>;
};