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
      switch ([%js.deep formation["Line"]]) {
      | Some((count, dist)) => `Line((count, dist))
      | None =>
        switch ([%js.deep formation["Circle"]]) {
        | Some((count, dist, center)) => `Circle((count, dist, center))
        | None => `Single
        }
      };
    <div className=Styles.column>
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
          disabled={
            switch (kind) {
            | `Line(_) => true
            | _ => false
            }
          }
          onClick={_ =>
            onChange({
              "Single": None,
              "Line": Some((3, 50.0)),
              "Circle": None,
            })
          }>
          line
        </button>
        {Styles.spacer(16)}
        <button
          className=colorButton
          disabled={
            switch (kind) {
            | `Circle(_) => true
            | _ => false
            }
          }
          onClick={_ =>
            onChange({
              "Single": None,
              "Line": None,
              "Circle": Some((5, 50.0, false)),
            })
          }>
          circle
        </button>
      </div>
      {
        let smallLabel =
          Css.(
            style([fontSize(px(Styles.Text.small)), fontWeight(`normal)])
          );
        switch (kind) {
        | `Single => React.null
        | `Line(count, dist) =>
          <div className=Styles.column>
            {Styles.spacer(8)}
            <div
              className={Styles.join([
                Styles.row,
                Css.(
                  style([
                    padding(px(8)),
                    border(px(2), `solid, Colors.accent),
                  ])
                ),
              ])}>
              {Styles.spacer(8)}
              <div className=smallLabel> {React.string("Count")} </div>
              {Styles.spacer(8)}
              <Ui.NumInput
                value={float_of_int(count)}
                min=2
                onChange={count =>
                  onChange({
                    "Single": None,
                    "Line": Some((int_of_float(count), dist)),
                    "Circle": None,
                  })
                }
              />
              {Styles.spacer(16)}
              <div className=smallLabel> {React.string("Spacing")} </div>
              {Styles.spacer(8)}
              <Ui.NumInput
                value=dist
                min=2
                onChange={dist =>
                  onChange({
                    "Single": None,
                    "Line": Some((count, dist)),
                    "Circle": None,
                  })
                }
              />
              {Styles.spacer(8)}
            </div>
          </div>
        | `Circle(count, dist, center) =>
          <div className=Styles.column>
            {Styles.spacer(8)}
            <div
              className={Styles.join([
                Styles.row,
                Css.(
                  style([
                    padding(px(8)),
                    border(px(2), `solid, Colors.accent),
                  ])
                ),
              ])}>
              {Styles.spacer(8)}
              <div className=smallLabel> {React.string("Count")} </div>
              {Styles.spacer(8)}
              <Ui.NumInput
                value={float_of_int(count)}
                min=2
                onChange={count =>
                  onChange({
                    "Single": None,
                    "Circle": Some((int_of_float(count), dist, center)),
                    "Line": None,
                  })
                }
              />
              {Styles.spacer(16)}
              <div className=smallLabel> {React.string("Spacing")} </div>
              {Styles.spacer(8)}
              <Ui.NumInput
                value=dist
                min=2
                onChange={dist =>
                  onChange({
                    "Single": None,
                    "Circle": Some((count, dist, center)),
                    "Line": None,
                  })
                }
              />
              {Styles.spacer(8)}
            </div>
          </div>
        };
      }
    </div>;
  };
};

let toAngleSpread = (t0, t1) => {
  let angle = (t0 +. t1) /. 2.0;
  let spread = abs_float(angle -. t0) *. 2.0;
  (angle, spread);
};

let fromAngleSpread = (angle, spread) => {
  (angle -. spread /. 2.0, angle +. spread /. 2.0);
};

let fromRad = r => r *. 180.0 /. Js.Math._PI;
let toRad = d => d /. 180.0 *. Js.Math._PI;

module LightMod = {
  [@react.component]
  let make = (~light, ~onChange) => {
    let kind = [%js.deep light##kind["Point"]];
    let (angle, spread) = toAngleSpread(kind##t0, kind##t1);
    let modified =
      abs_float(kind##t0 +. Js.Math._PI) > 0.000001
      || abs_float(kind##t1 -. Js.Math._PI) > 0.00001;
    <div className=Styles.column>
      <div className=Styles.row>
        <button
          className={Styles.toggleButton(kind##offset != 0.0)}
          onClick={_ =>
            onChange(
              [%js.deep
                light["kind"]["Point"]["offset"].map(offset =>
                  offset == 0.0 ? 10.0 : 0.0
                )
              ],
              true,
            )
          }>
          <svg width="50" height="50">
            <defs>
              <radialGradient id="myGradient">
                <stop offset="10%" stopColor="white" />
                <stop offset="50%" stopColor="black" />
              </radialGradient>
            </defs>
            <circle cx="25" cy="25" r="50" fill="url('#myGradient')" />
            <circle cx="25" cy="25" r="5" fill="black" />
          </svg>
        </button>
        {kind##offset != 0.0
           ? <div className=Css.(style([padding(px(0))]))>
               <Ui.Slider
                 min=1
                 max=500.0
                 step=1.0
                 value=kind##offset
                 onChange={offset =>
                   onChange(
                     [%js.deep
                       light["kind"]["Point"]["offset"].replace(offset)
                     ],
                     false,
                   )
                 }
               />
             </div>
           : React.null}
      </div>
      {Styles.spacer(16)}
      <div className=Styles.row>
        <button
          className={Styles.toggleButton(modified)}
          onClick={_ =>
            if (modified) {
              onChange(
                [%js.deep
                  light["kind"]["Point"]["t0"].replace(-. Js.Math._PI)["kind"]["Point"]["t1"].
                    replace(
                    Js.Math._PI,
                  )
                ],
                true,
              );
            } else {
              onChange(
                [%js.deep
                  light["kind"]["Point"]["t0"].replace(-. Js.Math._PI /. 5.0)["kind"]["Point"]["t1"].
                    replace(
                    Js.Math._PI /. 5.0,
                  )
                ],
                true,
              );
            }
          }>
          <svg width="50" height="50">
            <defs>
              <radialGradient id="myGradient">
                <stop offset="10%" stopColor="white" />
                <stop offset="50%" stopColor="black" />
              </radialGradient>
            </defs>
            <circle cx="15" cy="20" r="50" fill="url('#myGradient')" />
            <path fill="black" d="M0 0 L50 0 L50 20 L15 20 L50 50 L0 50z" />
          </svg>
        </button>
        {modified
           ? <div className=Styles.column>
               <Ui.Slider
                 min=0
                 max=360.0
                 step=1.0
                 value={fromRad(angle)}
                 onChange={angle => {
                   let (t0, t1) = fromAngleSpread(toRad(angle), spread);

                   onChange(
                     [%js.deep
                       light["kind"]["Point"]["t0"].replace(t0)["kind"]["Point"]["t1"].
                         replace(
                         t1,
                       )
                     ],
                     false,
                   );
                 }}
               />
               <Ui.Slider
                 min=0
                 max=359.0
                 step=1.0
                 value={fromRad(spread)}
                 onChange={spread => {
                   let (t0, t1) = fromAngleSpread(angle, toRad(spread));

                   onChange(
                     [%js.deep
                       light["kind"]["Point"]["t0"].replace(t0)["kind"]["Point"]["t1"].
                         replace(
                         t1,
                       )
                     ],
                     false,
                   );
                 }}
               />
             </div>
           : React.null}
      </div>
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
    {Styles.spacer(16)}
    <LightMod
      light={Array.get(config##lights, 0)}
      onChange={(light, checkpoint) =>
        onChange(
          [%js.deep
            config["lights"].map(lights => {
              let lights = Js.Array.copy(lights);
              lights[0] = light;
              lights;
            })
          ],
          checkpoint,
        )
      }
    />
  </div>;
};