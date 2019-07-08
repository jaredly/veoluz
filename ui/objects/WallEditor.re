open Ui;

let wallType = kind =>
  if ([%js.deep kind["Line"]] != None) {
    "Line";
  } else if ([%js.deep kind["Parabola"]] != None) {
    "Parabola";
  } else {
    "Arc";
  };

[@react.component]
let make =
    (~wasm, ~selected, ~wall, ~index, ~onChange, ~onRemove, ~updateUi, ~ui) => {
  <div
    className=Css.(
      style(
        [
          // cursor(`pointer),
          padding2(~v=px(8), ~h=`zero),
          borderBottom(px(1), `solid, hex("ddd")),
          hover([backgroundColor(Colors.buttonHover)]),
        ]
        @ (
          selected
            ? [
              outline(px(2), `solid, Colors.accent),
              hover([outline(px(2), `solid, Colors.accent)]),
            ]
            : []
        ),
      )
    )
    onMouseOver={evt => wasm##hover_wall(index)}>
    <div
      className=Css.(
        style([
          display(`flex),
          paddingRight(px(8)),
          cursor(`pointer),
          // justifyContent(`spaceBetween),
          alignItems(`center),
        ])
      )
      onClick={evt =>
        if (selected) {
          updateUi([%js.deep ui["selection"].replace(Js.null)]);
        } else {
          wasm##set_active_wall(index);
        }
      }>
      {selected ? <IonIcons.ArrowDown fontSize="14px" /> : <IonIcons.ArrowRight fontSize="14px" />}
      {Styles.spacer(4)}
      <div
        className=Css.(style([fontWeight(`medium), flex(1), fontSize(px(12))]))>
        {React.string(
            "Wall #" ++ string_of_int(index) ++ " " ++ wallType(wall##kind),
          )}
      </div>
      <button
        onClick={evt => {
          ReactEvent.Mouse.stopPropagation(evt);
          onChange([%js.deep wall["hide"].replace(!wall##hide)]);
        }}>
        {React.string(wall##hide ? "Show" : "Hide")}
      </button>
    </div>
    {selected
        ? <div className=Styles.join([Styles.column, Css.(style([padding2(~v=`zero, ~h=px(8))]))])>
            {Styles.spacer(8)}
            <div>
              {React.string("Absorb")}
              <Slider
                min=0
                max=1.0
                step=0.01
                value={wall##properties##absorb}
                onChange={absorb => {
                  let wall = [%js.deep
                    wall["properties"]["absorb"].replace(absorb)
                  ];
                  onChange(wall);
                }}
              />
            </div>
            {Styles.spacer(8)}
            <div>
              {React.string("Reflect vs Refract")}
              <Slider
                min=0
                max=1.0
                step=0.01
                value={wall##properties##reflect}
                onChange={reflect => {
                  let wall = [%js.deep
                    wall["properties"]["reflect"].replace(reflect)
                  ];
                  onChange(wall);
                }}
              />
            </div>
            {Styles.spacer(8)}
            <div>
              {React.string("Index of Refraction")}
              <LogSlider
                min=0
                max=5.0
                step=0.01
                value={wall##properties##refraction}
                onChange={value => {
                  let wall = [%js.deep
                    wall["properties"]["refraction"].replace(value)
                  ];
                  onChange(wall);
                }}
              />
            </div>
            {Styles.spacer(8)}
            <div>
              {React.string("Roughness")}
              <Slider
                min=0
                max=1.0
                step=0.01
                value={wall##properties##roughness}
                onChange={value => {
                  let wall = [%js.deep
                    wall["properties"]["roughness"].replace(value)
                  ];
                  onChange(wall);
                }}
              />
            </div>
            {Styles.spacer(8)}
            <button onClick={evt => onRemove()}>
              {React.string("Delete")}
            </button>
          </div>
        : React.null}
  </div>;
};
