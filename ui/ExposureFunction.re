let btn = Css.(style([disabled([backgroundColor(Colors.accent)])]));
[@react.component]
let make = (~config: Rust.config, ~update, ~wasm: Rust.wasm) => {
  let (isOpen, setOpen) = Hooks.useState(false);

  let canvasRef = React.useRef(Js.Nullable.null);

  React.useEffect2(
    () => {
      if (isOpen) {
        switch (Js.Nullable.toOption(canvasRef->React.Ref.current)) {
        | None => ()
        | Some(canvas) =>
          Js.log2("Sending canvas", canvas);
          wasm##show_hist(canvas->Web.asCanvas);
        };
      } else {
        wasm##hide_hist();
      };
      None;
    },
    (isOpen, canvasRef->React.Ref.current),
  );

  <div className=Styles.column>
    // className=Styles.title

      <div
        onClick={_ => setOpen(!isOpen)}
        className={Styles.join([
          Styles.row,
          Css.(
            style([
              cursor(`pointer),
              alignItems(`center),
              padding(px(4)),
              hover([backgroundColor(Colors.button)]),
            ])
          ),
        ])}>
        {isOpen
           ? <IonIcons.ArrowDown fontSize="14px" />
           : <IonIcons.ArrowRight fontSize="14px" />}
        {Styles.spacer(4)}
        {React.string("Contrast & Colors")}
      </div>
      <div
        style={ReactDOMRe.Style.make(~display=isOpen ? "flex" : "none", ())}
        className=Styles.column>
        {Styles.spacer(8)}
        <div className=Styles.row>
          <button
            disabled={config##rendering##exposure##curve == "FourthRoot"}
            onClick={_evt => {
              let config = [%js.deep
                config["rendering"]["exposure"]["curve"].replace("FourthRoot")
              ];
              update(config, true);
            }}
            className=btn>
            {React.string("Low")}
          </button>
          {Styles.spacer(4)}
          <button
            disabled={config##rendering##exposure##curve == "SquareRoot"}
            onClick={_evt => {
              let config = [%js.deep
                config["rendering"]["exposure"]["curve"].replace("SquareRoot")
              ];
              update(config, true);
            }}
            className=btn>
            {React.string("Medium")}
          </button>
          {Styles.spacer(4)}
          <button
            disabled={config##rendering##exposure##curve == "Linear"}
            onClick={_evt => {
              let config = [%js.deep
                config["rendering"]["exposure"]["curve"].replace("Linear")
              ];
              update(config, true);
            }}
            className=btn>
            {React.string("High")}
          </button>
        </div>
        {Styles.spacer(8)}
        <div className=Styles.row>
          <button
            disabled={config##rendering##exposure##limits == Js.Null.empty}
            onClick={_ => {
              let config = [%js.deep
                config["rendering"]["exposure"]["limits"].replace(
                  Js.Null.empty,
                )
              ];
              update(config, true);
            }}
            className=btn>
            {React.string("Automatic")}
          </button>
          {Styles.spacer(4)}
          <button
            disabled={config##rendering##exposure##limits != Js.Null.empty}
            onClick={_ => {
              let config = [%js.deep
                config["rendering"]["exposure"]["limits"].replace(
                  Js.Null.return((0.0, 1.0)),
                )
              ];
              update(config, true);
            }}
            className=btn>
            {React.string("Manual")}
          </button>
        </div>
        {switch (Js.Null.toOption(config##rendering##exposure##limits)) {
         | None => React.null
         | Some((min, max)) =>
           <div className=Styles.column>
             {Styles.spacer(8)}
             <div
               className=Css.(
                 style([
                   position(`relative),
                   paddingBottom(px(12)),
                   marginBottom(px(8)),
                 ])
               )>
               <canvas
                 width="200px"
                 height="100px"
                 ref={ReactDOMRe.Ref.domRef(canvasRef)}
               />
               <ExposureControl
                 limits=(min, max)
                 wasm
                 config
                 update
                 width=200
               />
             </div>
             {Styles.spacer(8)}
             <div className=Styles.row>
               <Ui.NumInput
                 value=min
                 width=70
                 onChange={v => {
                   let config = [%js.deep
                     config["rendering"]["exposure"]["limits"].replace(
                       Js.Null.return((v, max)),
                     )
                   ];
                   update(config, false);
                 }}
               />
               {Styles.spacer(4)}
               <Ui.NumInput
                 width=70
                 value=max
                 onChange={v => {
                   let config = [%js.deep
                     config["rendering"]["exposure"]["limits"].replace(
                       Js.Null.return((min, v)),
                     )
                   ];
                   update(config, false);
                 }}
               />
             </div>
           </div>
         }}
        {Styles.spacer(8)}
        <div className=Styles.row>
          {switch ([%js.deep config##rendering##coloration["Rgb"]]) {
           | None => React.string("not rgb")
           | Some(rgb) =>
             //  className="color-picker-wrapper"
             <div
               style={ReactDOMRe.Style.make//  ~marginLeft="-13px",
                                           () //  ~width="10px",
 //  ~marginTop="2px",
                                           //  ~height="30px",
}>
               <ExposureControl.Colorpickr
                 color={ExposureControl.rgbToColor(rgb##background)}
                 onChange={color => {
                   Js.log2("Color", color);
                   let config = [%js.deep
                     config["rendering"]["coloration"]["Rgb"].map(rgb =>
                       switch (rgb) {
                       | None => None
                       | Some(v) =>
                         Some(
                           v["background"].replace(
                             ExposureControl.colorToRgb(color##color),
                           ),
                         )
                       }
                     )
                   ];
                   update(config, false);
                 }}
               />
             </div>
           }}
          {Styles.spacer(8)}
          {switch ([%js.deep config##rendering##coloration["Rgb"]]) {
           | None => React.string("not rgb")
           | Some(rgb) =>
             //  className="color-picker-wrapper"
             <div
               style={ReactDOMRe.Style.make//  ~marginLeft="-13px",
                                           () //  ~width="10px",
 //  ~marginTop="2px",
                                           //  ~height="30px",
}>
               <ExposureControl.Colorpickr
                 color={ExposureControl.rgbToColor(rgb##highlight)}
                 onChange={color => {
                   Js.log2("Color", color);
                   let config = [%js.deep
                     config["rendering"]["coloration"]["Rgb"].map(rgb =>
                       switch (rgb) {
                       | None => None
                       | Some(v) =>
                         Some(
                           v["highlight"].replace(
                             ExposureControl.colorToRgb(color##color),
                           ),
                         )
                       }
                     )
                   ];
                   update(config, false);
                 }}
               />
             </div>
           }}
        </div>
      </div>
    </div>;
};