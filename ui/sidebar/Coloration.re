type swatch = {
  name: string,
  background: (float, float, float),
  highlight: (float, float, float),
};

let palettes = [
  {
    name: "Default",
    background: (0.0, 0.0, 0.0),
    highlight: (255.0, 255.0, 255.0),
  },
  {
    name: "Warm",
    background: (0.0, 0.0, 0.0),
    highlight: (255.0, 244.0, 219.0),
  },
  {
    name: "Snowflake",
    background: (41.0, 60.0, 148.0),
    highlight: (255.0, 255.0, 255.0),
  },
  {
    name: "Fall",
    background: (148.0, 68.0, 41.0),
    highlight: (255.0, 255.0, 255.0),
  },
];

[@react.component]
let make = (~update, ~config) => {
  <div className=Styles.column>
    <div className=Styles.title> {React.string("Colors")} </div>
    {Styles.spacer(8)}
    <div className=Styles.row>
      {switch ([%js.deep config##rendering##coloration["Rgb"]]) {
       | None => React.string("not rgb")
       | Some(rgb) =>
         <div style={ReactDOMRe.Style.make()}>
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
         <div style={ReactDOMRe.Style.make()}>
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
  </div>;
};