let formationType = kind =>
  switch ([%js.deep kind["Single"]]) {
  | Some(_) => `Single
  | None =>
    switch ([%js.deep kind["Circle"]]) {
    | Some((count, dist, center)) => `Circle((count, dist, center))
    | None =>
      switch ([%js.deep kind["Line"]]) {
      | Some((count, dist)) => `Line((count, dist))
      | None => `Other
      }
    }
  };

// let btn = Css.(style([
//   disabled([backgroundColor(Colors.accent)])
// ]));

let btn =
  Css.(
    style([
      backgroundColor(`transparent),
      // textDecoration(`underline),
      disabled([
        textDecorationColor(Colors.accent),
        // backgroundColor(Colors.accent),
        textDecoration(`underline),
      ]),
    ])
  );

[@react.component]
let make = (~wasm, ~update, ~config) => {
  <div className=Styles.column>
    <div className=Styles.row>
      <div
        className=Css.(
          style([fontWeight(`bold), padding2(~v=px(8), ~h=`zero)])
        )>
        {React.string("Light formation")}
      </div>
      {Styles.spacer(8)}
      <button
        className=btn
        disabled={[%js.deep config##light_formation["Single"]] != None}
        onClick={_evt => {
          let config = [%js.deep
            config["light_formation"].replace({
              "Single": Some(Js.Null.empty),
              "Line": None,
              "Circle": None,
            })
          ];
          update(config, false);
        }}>
        {React.string("Single")}
      </button>
      {Styles.spacer(4)}
      <button
        className=btn
        disabled={[%js.deep config##light_formation["Circle"]] != None}
        onClick={_evt => {
          let config = [%js.deep
            config["light_formation"].replace({
              "Single": None,
              "Line": None,
              "Circle": Some((3, 50.0, false)),
            })
          ];
          update(config, false);
        }}>
        {React.string("Circle")}
      </button>
      {Styles.spacer(4)}
      <button
        className=btn
        disabled={[%js.deep config##light_formation["Line"]] != None}
        onClick={_evt => {
          let config = [%js.deep
            config["light_formation"].replace({
              "Single": None,
              "Line": Some((3, 50.0)),
              "Circle": None,
            })
          ];
          update(config, false);
        }}>
        {React.string("Line")}
      </button>
    </div>
    {switch (formationType(config##light_formation)) {
     | `Circle(count, dist, center) =>
       <div className=Styles.column>
         {Styles.spacer(8)}
         <div className=Styles.row>
           {React.string("Count")}
           {Styles.spacer(4)}
           <Ui.NumInput
             min=2
             max=30.0
             step=1.0
             value={float_of_int(count)}
             onChange={v => {
               let v = int_of_float(v);
               let config = [%js.deep
                 config["light_formation"].replace({
                   "Single": None,
                   "Line": None,
                   "Circle": Some((v, dist, center)),
                 })
               ];
               update(config, false);
             }}
           />
           {Styles.spacer(8)}
           {React.string("Radius")}
           {Styles.spacer(4)}
           <Ui.NumInput
             min=0
             max=300.0
             step=5.0
             value=dist
             onChange={v => {
               let config = [%js.deep
                 config["light_formation"].replace({
                   "Single": None,
                   "Line": None,
                   "Circle": Some((count, v, center)),
                 })
               ];
               update(config, false);
             }}
           />
         </div>
         {Styles.spacer(8)}
         <button
           className={Css.style([
             Css.backgroundColor(center ? Colors.accent : Colors.button),
             Css.alignSelf(`flexStart),
           ])}
           onClick={_evt => {
             let config = [%js.deep
               config["light_formation"].replace({
                 "Single": None,
                 "Line": None,
                 "Circle": Some((count, dist, !center)),
               })
             ];
             update(config, false);
           }}>
           {React.string("Center dot")}
         </button>
       </div>
     | `Line(count, dist) =>
       <div
         className={Styles.join([
           Styles.row,
           Css.(style([marginTop(px(8))])),
         ])}>
         {Styles.spacer(8)}
         {React.string("Count")}
         {Styles.spacer(4)}
         <Ui.NumInput
           min=2
           max=30.0
           step=1.0
           value={float_of_int(count)}
           onChange={v => {
             let v = int_of_float(v);
             let config = [%js.deep
               config["light_formation"].replace({
                 "Single": None,
                 "Circle": None,
                 "Line": Some((v, dist)),
               })
             ];
             update(config, false);
           }}
         />
         {Styles.spacer(8)}
         {React.string("Spacing")}
         {Styles.spacer(4)}
         <Ui.NumInput
           min=0
           max=300.0
           step=5.0
           value=dist
           onChange={v => {
             let config = [%js.deep
               config["light_formation"].replace({
                 "Single": None,
                 "Circle": None,
                 "Line": Some((count, v)),
               })
             ];
             update(config, false);
           }}
         />
       </div>
     | _ => React.null
     }}
  </div>;
};