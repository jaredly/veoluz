open Lets;

module NumInput = {
  [@react.component]
  let make = (~value, ~min, ~max, ~step, ~onChange) => {
    <input
      type_="number"
      min={min}
      max={Js.Float.toString(max)}
      value={Js.Float.toString(value)}
      step={step}
      onChange={evt => {
        let v = Js.Float.fromString(evt->ReactEvent.Form.target##value);
        onChange(v)
      }}
    />
  }

}

module LogSlider = {
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
        // value={Js.Float.toString(Js.Math.log(value))}
        value={Js.Float.toString(Js.Math.pow_float(~base=Js.Math._E, ~exp=value))}
        step={step}
        onChange={evt => {
          let v = Js.Float.fromString(evt->ReactEvent.Form.target##value);
          // onChange(Js.Math.pow_float(~base=Js.Math._E, ~exp=v))
          onChange(Js.Math.log(v))
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

module LightEditor = {
  [@react.component]
  let make = (~wasm, ~selected, ~light, ~index, ~onChange) => {
    <div
      className=Css.(style([
        padding(px(8)),
        cursor(`pointer),
        margin2(~v=px(8), ~h=px(0)),
      ] @ (
        selected
        ? [backgroundColor(hex("ddd"))]
        : []
      )))
      onMouseOver={evt => {
        // wasm##hover_wall(index)
        ()
      }}
      onClick={evt => {
        wasm##set_active_light(index);
        ()
      }}
    >
      <div className=Css.(style([
        fontWeight(`medium),
        fontSize(px(12)),

      ]))>
        {React.string("Light #" ++ string_of_int(index))}
      </div>
      {selected ? {
        let point = [%js.deep light##kind["Point"]];
        <div>
          {React.string("Offset")}
          <NumInput
            min={0}
            max={500.0}
            step={5.0}
            value={point##offset}
            onChange={offset => {
              onChange([%js.deep light["kind"]["Point"]["offset"].replace(offset)])
            }}
          />
          <br/>
          {React.string("x")}
          <NumInput
            min={-500}
            max={500.0}
            step={5.0}
            value={point##origin |> fst}
            onChange={x => {
              onChange([%js.deep light["kind"]["Point"]["origin"].map(((_, y)) => (x, y))])
            }}
          />
          {React.string("y")}
          <NumInput
            min={-500}
            max={500.0}
            step={5.0}
            value={point##origin |> snd}
            onChange={y => {
              onChange([%js.deep light["kind"]["Point"]["origin"].map(((x, _)) => (x, y))])
            }}
          />
          <br/>
          {React.string("t0")}
          <NumInput
            min={0}
            max={Js.Math._PI *. 2.0}
            step={0.1}
            value={point##t0}
            onChange={t0 => {
              onChange([%js.deep light["kind"]["Point"]["t0"].replace(t0)])
            }}
          />
          {React.string("t1")}
          <NumInput
            min={0}
            max={Js.Math._PI *. 2.0}
            step={0.1}
            value={point##t1}
            onChange={t1 => {
              onChange([%js.deep light["kind"]["Point"]["t1"].replace(t1)])
            }}
          />
        </div>
      } : React.null}
    </div>

  }

}

module WallEditor = {
  [@react.component]
  let make = (~wasm, ~selected, ~wall, ~index, ~onChange, ~onRemove) => {
    <div
      className=Css.(style([
        cursor(`pointer),
        padding2(~v=px(8), ~h=px(8)),
        borderBottom(px(1), `solid, hex("ddd")),
        hover([
          backgroundColor(hex("eee"))
        ])
      ] @ (
        selected
        ? [
          backgroundColor(hex("ddd")),
        hover([
          backgroundColor(hex("ddd"))
        ])
          ]
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
        display(`flex),
        justifyContent(`spaceBetween)
      ]))>
        <div className=Css.(style([
          fontWeight(`medium),
          fontSize(px(12)),

        ]))>
          {React.string("Wall #" ++ string_of_int(index))}
        </div>
        <button
          onClick={evt => {
            ReactEvent.Mouse.stopPropagation(evt);
              onChange([% js.deep wall["hide"].replace(!wall##hide)])
          }}
        >
          {React.string(wall##hide ? "Show" : "Hide")}
        </button>
      </div>
      {selected ? <div>
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
      <LogSlider
        min={0}
        max={5.0}
        step={0.01}
        value={wall##properties##refraction}
        onChange={value => {
            let wall = [%js.deep wall["properties"]["refraction"].replace(value)];
            onChange(wall)
        }}
      />
      <button
        onClick={evt => onRemove()}
      >
        {React.string("Delete")}
      </button>
      </div> : React.null}
    </div>
  }
}

[@react.component]
let make = (~ui: Rust.ui, ~config: Rust.config, ~update, ~updateUi, ~wasm: Rust.wasm) => {
  // Js.log(ui);
  // Js.log2("Config", config);

  <div
    className=Styles.control
    onMouseOver={evt => {
      wasm##show_ui();
    }}
    onMouseOut={evt => {
      wasm##hide_ui();
    }}
  >
    <div className=Styles.title>
      {React.string("Scene objects")}
    </div>
    <div>
      {config##lights->Belt.Array.mapWithIndex((i, light) => {
        <LightEditor
          wasm
          key={string_of_int(i)}
          light
          selected={switch (ui##selection->Js.nullToOption) {
            | None => false
            | Some(selection) => switch ([%js.deep selection["Light"]]) {
              | None => false
              | Some((lid, _)) => i == lid
            }
          }}
          index={i}
          onChange={light => {
            let config = [%js.deep config["lights"].map(lights => {
              let lights = Js.Array.copy(lights);
              lights[i] = light;
              lights
            })];
            update(config, false)
          }}
        />
      })->React.array}
    </div>
    <div className=Styles.row>
      <button
        onClick={_evt => updateUi([%js.deep ui["selection"].replace(Js.Null.return({"Adding": Some("Line"), "Multiple": None, "Light": None, "Wall": None}))])}
      >
        {React.string("Add line")}
      </button>
      (Styles.spacer(4))
      <button
        onClick={_evt => updateUi([%js.deep ui["selection"].replace(Js.Null.return({"Adding": Some("Parabola"), "Multiple": None, "Light": None, "Wall": None}))])}
      >
        {React.string("Add parabola")}
      </button>
      (Styles.spacer(4))
      <button
        onClick={_evt => updateUi([%js.deep ui["selection"].replace(Js.Null.return({"Adding": Some("Circle"), "Multiple": None, "Light": None, "Wall": None}))])}
      >
        {React.string("Add arc")}
      </button>
    </div>
    <div className=Css.(style([
      fontWeight(`bold),
      padding(px(8))
    ]))>
      {React.string("Walls")}
    </div>
    <div>
      {config##walls->Belt.Array.mapWithIndex((i, wall) => {
        <WallEditor
          key={string_of_int(i)}
          wasm
          wall
          selected={switch (ui##selection->Js.nullToOption) {
            | None => false
            | Some(selection) => switch ([%js.deep selection["Wall"]]) {
              | None => false
              | Some((wid, _)) => i == wid
            }
          }}
          index={i}
          onRemove={() => {
            let config = [%js.deep config["walls"].map(walls => {
              let walls = Js.Array.copy(walls);
              // Js.Array.removeFromInPlace(~pos=i, walls)->ignore;
              Js.Array.removeCountInPlace(~pos=i, ~count=1, walls)->ignore;
              walls
            })];
            update(config, true)
          }}
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
