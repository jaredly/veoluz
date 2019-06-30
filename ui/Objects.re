open Lets;

module Slider = {
  [@react.component]
  let make = (~value, ~min, ~max, ~step, ~onChange) => {
    <div>
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
      {React.string(Js.Float.toString(value))}
    </div>
  }
}

module WallEditor = {
  [@react.component]
  let make = (~wasm, ~wall, ~index, ~onChange) => {
    <div className=Css.(style([
      padding(px(8)),
      margin2(~v=px(8), ~h=px(0)),
    ]))
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
    </div>
  }
}

[@react.component]
let make = (~config: Rust.config, ~update, ~wasm: Rust.wasm) => {

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
