
module ExposureFunction = {
  let btn = Css.(style([disabled([backgroundColor(Colors.accent)])]));
  [@react.component]
  let make = (~config, ~update, ~wasm) => {
    let (isOpen, setOpen) = Hooks.useState(false);


    let canvasRef = React.useRef(Js.Nullable.null);

    React.useEffect1(
      () => {
        if (isOpen) {
          switch (Js.Nullable.toOption(canvasRef->React.Ref.current)) {
            | None => ()
            | Some(canvas) =>
            Js.log2("Sending canvas", canvas);
            wasm##show_hist(canvas->Web.asCanvas);
          }
        } else {
          wasm##hide_hist();
        };
        None;
      },
      [|isOpen|],
    );

    <div className=Styles.column>
      // className=Styles.title
      <div onClick={_ => setOpen(!isOpen)}
      className=Styles.join([Styles.row, Css.(style([
        cursor(`pointer),
        alignItems(`center),
        padding(px(4)),
        hover([
          backgroundColor(Colors.button)
        ])
      ]))])
      >
        {isOpen ? <IonIcons.ArrowDown fontSize="14px" /> : <IonIcons.ArrowRight fontSize="14px" />}
        {Styles.spacer(4)}
        {React.string("Exposure")}
      </div>
      <div
        style={ReactDOMRe.Style.make(~display=isOpen ? "flex" : "none", ())}
        className=Styles.column
      >
        {Styles.spacer(8)}
        <div className=Css.(style([position(`relative), paddingBottom(px(12)), marginBottom(px(8))]))>
          <canvas width="200px" height="100px" ref={ReactDOMRe.Ref.domRef(canvasRef)} />
          <ExposureControl wasm config update width=200 />
        </div>
        <div className=Styles.row>
          {switch ([%js.deep config##rendering##coloration["Rgb"]]) {
           | None => React.string("not rgb")
           | Some(rgb) =>
             <div
              //  className="color-picker-wrapper"
               style={ReactDOMRe.Style.make(
                //  ~width="10px",
                //  ~marginLeft="-13px",
                //  ~marginTop="2px",
                //  ~height="30px",
                 (),
               )}>
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
        {Styles.spacer(4)}
          <input
            type_="number"
            value={Js.Float.toString(config##rendering##exposure##min)}
            className=Css.(style([width(px(70))]))
            onChange={evt => {
              let v = evt->ReactEvent.Form.target##value->float_of_string;
              let config = [%js.deep
                config["rendering"]["exposure"]["min"].replace(v)
              ];
              update(config, false);
            }}
          />
        {Styles.spacer(8)}
          {switch ([%js.deep config##rendering##coloration["Rgb"]]) {
           | None => React.string("not rgb")
           | Some(rgb) =>
             <div
              //  className="color-picker-wrapper"
               style={ReactDOMRe.Style.make(
                //  ~width="10px",
                //  ~marginLeft="-13px",
                //  ~marginTop="2px",
                //  ~height="30px",
                 (),
               )}>
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
        {Styles.spacer(4)}
          <input
            type_="number"
            className=Css.(style([width(px(70))]))
            value={Js.Float.toString(config##rendering##exposure##max)}
            onChange={evt => {
              let v = evt->ReactEvent.Form.target##value->float_of_string;
              let config = [%js.deep
                config["rendering"]["exposure"]["max"].replace(v)
              ];
              update(config, false);
            }}
          />
        </div>
        {Styles.spacer(16)}
        <div className=Styles.row>
          <button
            disabled={config##rendering##exposure##curve == "FourthRoot"}
            onClick={_evt => {
              let config = [%js.deep
                config["rendering"]["exposure"]["curve"].replace("FourthRoot")
              ];
              update(config, false);
            }}
            className=btn>
            {React.string("Fourth Root")}
          </button>
          {Styles.spacer(4)}
          <button
            disabled={config##rendering##exposure##curve == "SquareRoot"}
            onClick={_evt => {
              let config = [%js.deep
                config["rendering"]["exposure"]["curve"].replace("SquareRoot")
              ];
              update(config, false);
            }}
            className=btn>
            {React.string("Square Root")}
          </button>
          {Styles.spacer(4)}
          <button
            disabled={config##rendering##exposure##curve == "Linear"}
            onClick={_evt => {
              let config = [%js.deep
                config["rendering"]["exposure"]["curve"].replace("Linear")
              ];
              update(config, false);
            }}
            className=btn>
            {React.string("Linear")}
          </button>
        </div>
      </div>
    </div>;
  };
};

module TransformEditor = {
  [@react.component]
  let make = (~config, ~update, ~wasm) => {
    <div
      className={Styles.join([
        Styles.control,
        Css.(style([display(`flex), flexDirection(`column)])),
      ])}>
      <div className=Styles.title> {React.string("Scene transforms")} </div>
      <div>
        {React.string("Rotational symmetry: ")}
        <input
          type_="number"
          min=0
          value={config##transform##rotational_symmetry |> string_of_int}
          max="30"
          onChange={evt => {
            let v = int_of_string(evt->ReactEvent.Form.target##value);
            let config = [%js.deep
              config["transform"]["rotational_symmetry"].replace(v)
            ];
            update(config, false);
          }}
        />
      </div>
      {Styles.spacer(8)}
      <div>
        <input
          type_="checkbox"
          checked={config##transform##reflection}
          onChange={evt => {
            let checked = evt->ReactEvent.Form.target##checked;
            let config = [%js.deep
              config["transform"]["reflection"].replace(checked)
            ];
            update(config, false);
          }}
        />
        {React.string(" Reflect over y axis")}
      </div>
      {Styles.spacer(8)}
      <div>
        {React.string("Center offset: ")}
        <input
          type_="number"
          className=Css.(style([width(px(50))]))
          value={config##rendering##center |> fst |> Js.Float.toString}
          onChange={evt => {
            let x = float_of_string(evt->ReactEvent.Form.target##value);
            let config = [%js.deep
              config["rendering"]["center"].map(((_, y)) => (x, y))
            ];
            update(config, false);
          }}
        />
        <input
          type_="number"
          value={config##rendering##center |> snd |> Js.Float.toString}
          className=Css.(style([width(px(50))]))
          onChange={evt => {
            let y = float_of_string(evt->ReactEvent.Form.target##value);
            let config = [%js.deep
              config["rendering"]["center"].map(((x, _)) => (x, y))
            ];
            update(config, false);
          }}
        />
      </div>
      {Styles.spacer(8)}
      <div>
        {React.string("Zoom: ")}
        <input
          type_="number"
          value={config##rendering##zoom |> Js.Float.toString}
          className=Css.(style([width(px(50))]))
          onChange={evt => {
            let zoom = float_of_string(evt->ReactEvent.Form.target##value);
            let config = [%js.deep config["rendering"]["zoom"].replace(zoom)];
            update(config, false);
          }}
        />
      </div>
      {Styles.spacer(8)}
      <ExposureFunction wasm config update />
    </div>;
  };
};
