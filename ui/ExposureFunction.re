let btn = Css.(style([disabled([backgroundColor(Colors.accent)])]));
[@react.component]
let make = (~config: Rust.config, ~update, ~wasm: Rust.wasm) => {
  // let (isOpen, setOpen) = Hooks.useState(false);

  let canvasRef = React.useRef(Js.Nullable.null);

  React.useEffect1(
    () => {
      // if (isOpen) {
      switch (Js.Nullable.toOption(canvasRef->React.Ref.current)) {
      | None => wasm##hide_hist()
      | Some(canvas) =>
        Js.log2("Sending canvas", canvas);
        wasm##show_hist(canvas->Web.asCanvas);
      };
      // } else {
      //   wasm##hide_hist();
      // };
      None;
    },
    [|canvasRef->React.Ref.current|],
  );

  <div className=Styles.column>
    <div className=Styles.title> {React.string("Contrast")} </div>
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
      // {Styles.spacer(4)}
      Styles.fullSpace
      <Tippy content="Manual exposure">
        <button
          className={Styles.join([
            Styles.toggleButton(
              config##rendering##exposure##limits != Js.Null.empty,
            ),
            Css.(style([borderStyle(`none)])),
          ])}
          onClick={_evt =>
            if (config##rendering##exposure##limits != Js.Null.empty) {
              let config = [%js.deep
                config["rendering"]["exposure"]["limits"].replace(
                  Js.Null.empty,
                )
              ];
              update(config, true);
            } else {
              let config = [%js.deep
                config["rendering"]["exposure"]["limits"].replace(
                  Js.Null.return((0.0, 1.0)),
                )
              ];
              update(config, true);
            }
          }>
          <IonIcons.Settings fontSize="20px" color="currentcolor" />
        </button>
      </Tippy>
    </div>
    {switch (Js.Null.toOption(config##rendering##exposure##limits)) {
     | None => React.null
     | Some((min, max)) =>
       <div className=Styles.column>
         {Styles.spacer(8)}
         <div className=Css.(style([fontSize(px(12)), margin(px(4))]))>
           {React.string(
              "Manual exposure control allows you to fine-tune the lights & darks and achieve a polished final image. However, as it does not automatically adjust, changing the scene configuration while manual control is enabled often results in over- or under-exposed images. It is recommended to only turn on manual exposure control once you are happy with the placement of walls, and are making final adjustments.",
            )}
         </div>
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
           <ExposureControl limits=(min, max) wasm config update width=200 />
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
    {Styles.spacer(16)}
    <Coloration update config />
    {Styles.spacer(8)}
  </div>;
};