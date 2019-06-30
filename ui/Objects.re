open Lets;

module Slider = {
  [@react.component]
  let make = (~value, ~min, ~max, ~step, ~onChange) => {
    <div className=Css.(style([
      display(`flex),
      flexDirection(`row)
    ]))>
      <input
        type_="range"
        min={min}
        max={Js.Float.toString(max)}
        value={Js.Float.toString(value)}
        step={step}
        onChange={evt => {
          let v = Js.Float.fromString(evt->ReactEvent.Form.target##value);
          onChange(v)
        }}
      />
      <div className=Css.(style([
        fontSize(px(8)),
        width(px(20))
      ]))>
      {React.string(Js.Float.toString(value))}
      </div>
    </div>
  }
}

module WallEditor = {
  [@react.component]
  let make = (~wasm, ~selected, ~wall, ~index, ~onChange) => {
    <div
      className=Css.(style([
        padding(px(8)),
        margin2(~v=px(8), ~h=px(0)),
      ] @ (
        selected
        ? [backgroundColor(hex("ddd"))]
        : []
      )))
      onMouseOver={evt => {
        wasm##hover_wall(index)
      }}
      onClick={evt => {
        wasm##set_active_wall(index);
      }}
    >
      <div className=Css.(style([
        fontWeight(`medium),
        fontSize(px(12)),

      ]))>
        {React.string("Wall #" ++ string_of_int(index))}
      </div>
      {React.string("Absorb")}
      <Slider
        min={0}
        max={1.0}
        step={0.01}
        value={wall##properties##absorb}
        onChange={absorb => {
            let wall = [%js.deep wall["properties"]["absorb"].replace(absorb)];
            onChange(wall)
        }}
      />

      {React.string("Reflect vs Refract")}
      <Slider
        min={0}
        max={1.0}
        step={0.01}
        value={wall##properties##reflect}
        onChange={reflect => {
            let wall = [%js.deep wall["properties"]["reflect"].replace(reflect)];
            onChange(wall)
        }}
      />

      {React.string("Index of Refraction")}
      <Slider
        min={0}
        max={5.0}
        step={0.01}
        value={wall##properties##refraction}
        onChange={value => {
            let wall = [%js.deep wall["properties"]["refraction"].replace(value)];
            onChange(wall)
        }}
      />
    </div>
  }
}

[@react.component]
let make = (~ui: Rust.ui, ~config: Rust.config, ~update, ~wasm: Rust.wasm) => {
  Js.log(ui);

  <div
    onMouseOver={evt => {
      wasm##show_ui();
    }}
    onMouseOut={evt => {
      wasm##hide_ui();
    }}
  >
    <div>
    </div>
    <div>
      {config##walls->Belt.Array.mapWithIndex((i, wall) => {
        <WallEditor
          wasm
          wall
          selected={switch (ui##selection->Js.nullToOption) {
            | None => false
            | Some(selection) => switch ([%js.deep selection["Wall"]]->Js.nullToOption) {
              | None => false
              | Some((wid, _)) => i == wid
            }
          }}
          index={i}
          onChange={wall => {
            let config = [%js.deep config["walls"].map(walls => {
              let walls = Js.Array.copy(walls);
              walls[i] = wall;
              walls
            })];
            update(config, false)
          }}
        />
      })->React.array}
    </div>
  </div>
};
