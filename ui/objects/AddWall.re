let btn = Css.(style([disabled([backgroundColor(Colors.accent)])]));

[@react.component]
let make = (~ui, ~updateUi) => {
  <div className=Styles.row>
    <button
      className=btn
      disabled={
        switch (Js.nullToOption(ui##selection)) {
        | Some(obj) => [%js.deep obj["Adding"]] == Some("Line")
        | _ => false
        }
      }
      onClick={_evt =>
        updateUi(
          [%js.deep
            ui["selection"].replace(
              Js.Null.return({
                "Adding": Some("Line"),
                "Multiple": None,
                "Light": None,
                "Wall": None,
              }),
            )
          ],
        )
      }>
      {React.string("Add line")}
    </button>
    {Styles.spacer(4)}
    <button
      className=btn
      disabled={
        switch (Js.nullToOption(ui##selection)) {
        | Some(obj) => [%js.deep obj["Adding"]] == Some("Parabola")
        | _ => false
        }
      }
      onClick={_evt =>
        updateUi(
          [%js.deep
            ui["selection"].replace(
              Js.Null.return({
                "Adding": Some("Parabola"),
                "Multiple": None,
                "Light": None,
                "Wall": None,
              }),
            )
          ],
        )
      }>
      {React.string("Add parabola")}
    </button>
    {Styles.spacer(4)}
    <button
      className=btn
      disabled={
        switch (Js.nullToOption(ui##selection)) {
        | Some(obj) => [%js.deep obj["Adding"]] == Some("Circle")
        | _ => false
        }
      }
      onClick={_evt =>
        updateUi(
          [%js.deep
            ui["selection"].replace(
              Js.Null.return({
                "Adding": Some("Circle"),
                "Multiple": None,
                "Light": None,
                "Wall": None,
              }),
            )
          ],
        )
      }>
      {React.string("Add arc")}
    </button>
  </div>;
};