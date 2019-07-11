module Formation = {
  let size = 50.0;
  let single =
    <svg
      style={ReactDOMRe.Style.make(~backgroundColor="black", ())}
      width={Js.Float.toString(size)}
      height={Js.Float.toString(size)}>
      <circle
        cx={Js.Float.toString(size /. 2.0)}
        cy={Js.Float.toString(size /. 2.0)}
        r="5.0"
        fill="white"
      />
    </svg>;

  let circle = {
    let count = 5;
    let pi = 3.1415;
    let scale = pi *. 2.0 /. float_of_int(count);
    let items =
      RotationSym.many(
        count,
        i => {
          let c = size /. 2.0;
          let rad = 15.0;
          let angle = float_of_int(i) *. scale -. pi /. 2.0;

          <circle
            cx={Js.Float.toString(c +. cos(angle) *. rad)}
            cy={Js.Float.toString(c +. sin(angle) *. rad)}
            r="5"
            fill="white"
          />;
        },
      );
    <svg
      style={ReactDOMRe.Style.make(~backgroundColor="black", ())}
      width={Js.Float.toString(size)}
      height={Js.Float.toString(size)}>
      {React.array(Array.of_list(items))}
    </svg>;
  };

  let line = {
    <svg
      style={ReactDOMRe.Style.make(~backgroundColor="black", ())}
      width={Js.Float.toString(size)}
      height={Js.Float.toString(size)}>
      <circle
        cx={Js.Float.toString(size /. 2.0)}
        cy={Js.Float.toString(size /. 2.0)}
        r="5"
        fill="white"
      />
      <circle
        cx={Js.Float.toString(size /. 2.0 -. 15.)}
        cy={Js.Float.toString(size /. 2.0)}
        r="5"
        fill="white"
      />
      <circle
        cx={Js.Float.toString(size /. 2.0 +. 15.)}
        cy={Js.Float.toString(size /. 2.0)}
        r="5"
        fill="white"
      />
    </svg>;
  };

  let colorButton =
    Css.(
      style([
        padding(px(4)),
        backgroundColor(`transparent),
        backgroundColor(hex("fff")),
        hover([backgroundColor(Colors.button)]),
        disabled([
          backgroundColor(Colors.accent),
          hover([backgroundColor(Colors.accent)]),
        ]),
      ])
    );

  [@react.component]
  let make = (~formation, ~onChange) => {
    let kind =
      if ([%js.deep formation["Single"] != None]) {
        `Single;
      } else if ([%js.deep formation["Line"]] != None) {
        `Line;
      } else {
        `Circle;
      };
    <div className=Styles.row>
      <button
        className=colorButton
        disabled={kind == `Single}
        onClick={_ =>
          onChange({
            "Single": Some(Js.Null.empty),
            "Line": None,
            "Circle": None,
          })
        }>
        single
      </button>
      {Styles.spacer(16)}
      <button
        className=colorButton
        disabled={kind == `Line}
        onClick={_ =>
          onChange({"Single": None, "Line": Some((3, 50.0)), "Circle": None})
        }>
        line
      </button>
      {Styles.spacer(16)}
      <button
        className=colorButton
        disabled={kind == `Circle}
        onClick={_ =>
          onChange({
            "Single": None,
            "Line": None,
            "Circle": Some((5, 50.0, false)),
          })
        }>
        circle
      </button>
    </div>;
  };
};

[@react.component]
let make = (~config: Rust.config, ~onChange) => {
  <div className=Styles.column>
    <div className=Styles.title> {React.string("Light source")} </div>
    <Formation
      formation=config##light_formation
      onChange={lf =>
        onChange([%js.deep config["light_formation"].replace(lf)], true)
      }
    />
  </div>;
};