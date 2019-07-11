open Lets;

// [%bs.raw {|require("@mapbox/react-colorpickr/dist/colorpickr.css")|}];

type location;
[@bs.val] external location: location = "";

[@bs.set] external setHash: (location, string) => unit = "hash";

// module Colorpickr = {
//   type color = {. "r": float, "g": float, "b": float};
//   [@bs.module "@mapbox/react-colorpickr"]
//   [@react.component]
//   external make: (~onChange: (color) => unit) => React.element = "default";
// }

[%bs.raw {|require('rc-color-picker/assets/index.css')|}];

open Types;

let genId = () =>
  Js.Math.random()
  ->Js.Float.toStringWithRadix(~radix=36)
  ->Js.String2.sliceToEnd(~from=2);
let genId = () => genId() ++ genId();

let newScene = scene => {
  let id = genId();
  let created = Js.Date.now();
  let fullId = created->Js.Float.toString ++ ":" ++ id;
  {...scene, id: fullId, modified: created, created};
};

/**
Behavior:
- on first load, get the data from the hash, might be async
- on hash change that I initiate,
- to detect hashchanges I don't initiate, should use a ref probably. Update the ref then set the hash
  */

let hashIt: string => string = [%bs.raw
  {|
function(input) {
    var hash = 0;
    if (input.length == 0) {
        return hash;
    }
    for (var i = 0; i < input.length; i++) {
        var char = input.charCodeAt(i);
        hash = ((hash<<5)-hash)+char;
        hash = hash & hash; // Convert to 32bit integer
    }
    return hash;
}
|}
];

let anyHash = data => {
  hashIt(Js.Json.stringifyAny(data)->Opt.force);
};

let debounced = (fn, time) => {
  let timer = ref(None);
  arg => {
    switch (timer^) {
    | None => ()
    | Some(timer) => Js.Global.clearTimeout(timer)
    };

    timer := Some(Js.Global.setTimeout(() => fn(arg), time));
  };
};

// let interestingDefault: Rust.config = [%bs.raw
//   {|
// {"walls":[{"kind":{"Parabola":{"a":-0.00297538,"left":-87.706406,"right":37.56985,"transform":{"rotation":[-0.92809707,0.37233835],"translation":[46.11507,-51.88139]}}},"properties":{"absorb":0,"reflect":0,"roughness":0,"refraction":0.35065687},"hide":false},{"kind":{"Line":{"a":[164,-18],"b":[113.16666,-11.666672]}},"properties":{"absorb":0,"reflect":1,"roughness":0,"refraction":0.5},"hide":false},{"kind":{"Circle":[{"radius":25.632011},[210,9],-1.2120256,1.2890245]},"properties":{"absorb":0,"reflect":0.45,"roughness":0,"refraction":0.45742485},"hide":false}],"lights":[{"kind":{"Point":{"offset":0,"origin":[0,0],"t0":-3.1415927,"t1":3.1415927}},"brightness":1}],"transform":{"rotational_symmetry":5,"reflection":true},"rendering":{"exposure":{"curve":"SquareRoot","min":0.028320312,"max":0.38378906},"coloration":{"Rgb":{"background":[0,0,0],"highlight":[255,242,217]}},"width":1024,"height":576,"center":[1,0],"zoom":1}}
// |}
// ];

module Router = {
  let loadHash = (~hash, ~wasm: Rust.wasm, ~onLoad) =>
    if (Js.String2.startsWith(hash, "#id=")) {
      let id = Js.String2.sliceToEnd(hash, ~from=4);
      let%Async.Consume config = Web.LocalForage.getItem(id);
      switch (config->Js.toOption) {
      | None => ()
      | Some(config) => onLoad((Some(id), config))
      };
    } else if (String.length(hash) > 1) {
      let config =
        wasm##parse_url_config(hash->Js.String2.sliceToEnd(~from=1))
        ->Js.toOption;
      switch (config) {
      | None => ()
      | Some(config) => onLoad((None, config))
      };
    } else {
      // ermm maybe not a reset? idk.
      onLoad((None, wasm##blank_config()));
    };

  let updateId = (set, id) => {
    let hash = "id=" ++ id;
    set(hash);
    Web.Location.setHash(hash);
  };
  let permalink = (set, hash) => {
    set(hash);
    Web.Location.setHash(hash);
  };

  let useRouter = (~wasm: Rust.wasm, ~onLoad) => {
    let prevHash = React.useRef(None);
    let hash = Hooks.useHash();
    React.useEffect2(
      () => {
        if (prevHash->React.Ref.current != Some(hash)) {
          prevHash->React.Ref.setCurrent(Some(hash));
          loadHash(~hash, ~wasm, ~onLoad);
        };
        None;
      },
      (hash, prevHash->React.Ref.current),
    );

    React.useCallback(newHash =>
      prevHash->React.Ref.setCurrent(Some(newHash))
    );
    // updateId = id => {
    //   let hash = "id=" ++ id;
    //   prevHash->React.Ref.setCurrent(Some(hash));
    //   Web.Location.setHash(hash);
    // }
    // ~permalink=hash => {
    //   prevHash->React.Ref.setCurrent(Some(hash));
    //   Web.Location.setHash(hash);
    // },
  };
};

let downloadZip: (string, Web.canvas, Rust.config) => unit = [%bs.raw
  {|
(function(title, canvas, config) {
  var JSZip = require('jszip');
  var FileSaver = require('file-saver');
  var zip = new JSZip();

  // var img = zip.folder("images");
  // img.file("smile.gif", imgData, {base64: true});

  canvas.toBlob(blob => {
    zip.file("image.png", blob);
    zip.file("config.json", JSON.stringify(config))
    zip.generateAsync({type:"blob"}).then(function(content) {
      FileSaver.saveAs(content, title + ".zip");
    });
  }, 'image/png')
})
|}
];

let downloadCanvas: (string, Web.canvas) => unit = [%bs.raw
  {|
(function(title, canvas) {
  var a = document.createElement('a');
  a.download = title + ".png";
  a.href = canvas.toDataURL('image/png')
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
})
|}
];

let downloadFile: (string, string) => unit = [%bs.raw
  {|
(function(title, data) {
  var a = document.createElement('a');
  a.download = title + ".json";
  a.href = 'data:application/json,' + data;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
})
|}
];

Css.(
  global(
    "body",
    [
      backgroundColor(Colors.background),
      color(Colors.text),
      fontSize(px(Styles.Text.normal)),
      margin(px(0)),
    ],
  )
);

Css.(
  global(
    "input",
    [
      backgroundColor(Colors.control),
      color(Colors.text),
      fontSize(px(Styles.Text.small)),
    ],
  )
);

Css.(
  global(
    "button",
    [
      // backgroundColor(Colors.button),
      cursor(`pointer),
      fontSize(px(Styles.Text.small)),
      color(Colors.text),
      padding2(~v=px(4), ~h=px(8)),
      borderStyle(`none),
      backgroundColor(Colors.button),
      borderRadius(px(4)),
      hover([backgroundColor(Colors.buttonHover)]),
      disabled([backgroundColor(`transparent), cursor(`default)]),
    ],
  )
);

module Inner = {
  type state = {
    directory,
    hoverUrl: option(string),
    current: option(string),
    config: Rust.config,
    ui: Rust.ui,
  };

  [@react.component]
  let make = (~wasm: Rust.wasm, ~directory, ~blank) => {
    let router = ref(_ignored => ());

    let (state, dispatch) =
      React.useReducer(
        (state, action) =>
          switch (action) {
          | `Hover(url) => {...state, hoverUrl: Some(url)}
          | `Unhover => {...state, hoverUrl: None}
          | `Route(parentId, parentConfig) => {
              ...state,
              current: parentId,
              config: parentConfig,
            }
          | `UpdateTags(tags) =>
            // TODO safe the tags too yall
            {
              ...state,
              directory: {
                ...state.directory,
                tags,
              },
            }
          | `SaveInPlace((scene: scene)) => {
              ...state,
              directory: {
                ...state.directory,
                scenes:
                  state.directory.scenes
                  ->Belt.Map.String.set(scene.id, scene),
              },
            }
          | `Save((scene: scene)) =>
            (router^)->Router.updateId(scene.id);
            {
              ...state,
              directory: {
                ...state.directory,
                scenes:
                  state.directory.scenes
                  ->Belt.Map.String.set(scene.id, scene),
              },
              current: Some(scene.id),
            };
          | `Update(config, ui) => {...state, config, ui}
          | `Select(id) =>
            (router^)->Router.updateId(id);
            // updateId(id);
            {...state, current: Some(id)};
          },
        {
          directory,
          current: None,
          config: blank,
          ui: Rust.blankUi,
          hoverUrl: None,
        },
      );

    React.useEffect0(() => {
      // Js.log3("Setting up here", anyHash(state.config), state.config);
      let update =
        debounced(
          ((config, ui)) =>
            // configRef->React.Ref.setCurrent(config);
            dispatch(`Update((config, ui))),
          200,
        );
      wasm##setup(state.config, (config, ui)
        // Prevent a render loop
        // Js.log("Setting current from wasm (TODO debounce)");
        // configRef->React.Ref.setCurrent(config);
        // dispatch(`Update(config))
        => update((config, ui)));
      None;
    });

    router :=
      Router.useRouter(
        ~wasm,
        ~onLoad=((id, config)) => {
          // Js.log("Router log");
          // This upgrades the schema if needed
          let config = wasm##restore(config);
          Js.log2("Loading config", config);
          dispatch(`Route((id, config)));
        },
      );

    Hooks.useOnChange(
      state.directory,
      directory => {
        Js.log("Directory changed -- saving");
        let%Async.Consume () = ScenePicker.saveSceneInfo(directory);
        ();
      },
    )
    ->ignore;

    let onSaveScene =
      React.useCallback1(
        (scene: scene) => {
          let scene =
            scene.id == ""
              ? newScene(scene) : {...scene, modified: Js.Date.now()};
          let canvas = Web.documentGetElementById("drawing")->Web.asCanvas;
          canvas->Web.toBlob(blob => {
            let%Async.Consume () =
              Web.LocalForage.setItem(scene.id ++ ":image", blob);
            let%Async.Consume () =
              Web.LocalForage.setItem(scene.id, state.config);
            dispatch(`Save(scene));
            ();
          });
        },
        [|state.config|],
      );

    let update = (config, checkpoint) => {
      wasm##update(config, checkpoint);
      dispatch(`Update((config, state.ui)));
    };

    <div
      className=Css.(
        style([
          display(`flex),
          flexDirection(`row),
          alignItems(`stretch),
          maxWidth(px(1300)),
          margin2(~v=`zero, ~h=`auto),
          height(`vh(100.0)),
          // flexWrap(`wrap),
        ])
      )>
      <div
        className=Css.(
          style([
            position(`relative),
            display(`flex),
            flexDirection(`column),
            // overflow(`hidden),
            alignItems(`stretch),
          ])
        )>
        <div className=Css.(style([flexShrink(0), position(`relative)]))>
          <canvas
            id="drawing"
            width="600"
            height="600"
            className=Css.(style([]))
          />
          {switch (state.hoverUrl) {
           | None => React.null
           | Some(url) =>
             <img
               src=url
               className=Css.(
                 style([
                   backgroundColor(black),
                   position(`absolute),
                   top(px(0)),
                   pointerEvents(`none),
                   left(px(0)),
                 ])
               )
             />
           }}
        </div>
        <div
          className=Css.(
            style([color(white), fontSize(px(Styles.Text.small))])
          )>
          {React.string("Rays: ")}
          <span id="total_rays" />
          {React.string(" Rays/second: ")}
          <span id="fps" />
        </div>
        <MiniScenePicker
          onUpdateTags={tags => dispatch(`UpdateTags(tags))}
          directory={state.directory}
          onChangeScene={scene => dispatch(`SaveInPlace(scene))}
          onClearScene={() => {
            let _ = wasm##restore(wasm##blank_config());
            ();
          }}
          onSaveScene
          current={state.current}
          hover={url => dispatch(`Hover(url))}
          unHover={() => dispatch(`Unhover)}
          onExample={config => {
            let _ = wasm##restore(config);
            ();
          }}
          // Js.log3("Resetting", anyHash(config), config);
          // let _config = wasm##restore(config);
          // (router^)->Router.updateId(id);
          // dispatch(`Select(id));
          onSelect={(id, config) => {
            Js.log3("Resetting", anyHash(config), config);
            let _config = wasm##restore(config);
            (router^)->Router.updateId(id);
            dispatch(`Select(id));
          }}
        />
      </div>
      <div
        className=Css.(
          style([
            margin2(~h=px(8), ~v=px(8)),
            flex(1),
            display(`flex),
            flexDirection(`column),
            minHeight(px(200)),
          ])
        )>
        {
          let currentScene =
            switch (state.current) {
            | None => None
            | Some(key) => state.directory.scenes->Belt.Map.String.get(key)
            };

          <SceneForm
            ui={state.ui}
            directory={state.directory}
            onUpdateTags={tags => dispatch(`UpdateTags(tags))}
            scene=?currentScene
            wasm
            onPermalink={() =>
              (router^)
              ->Router.permalink(wasm##serialize_url_config(state.config))
            }
            onDownloadZip={() => {
              let title =
                switch (currentScene) {
                | Some({title: Some(title)}) => title
                | Some({created}) =>
                  Js.Date.toISOString(Js.Date.fromFloat(created))
                | _ => Js.Date.toISOString(Js.Date.make())
                };
              downloadZip(
                title,
                Web.documentGetElementById("drawing")->Web.asCanvas,
                state.config,
              );
            }}
            onDownload={() => {
              let title =
                switch (currentScene) {
                | Some({title: Some(title)}) => title
                | Some({created}) =>
                  Js.Date.toISOString(Js.Date.fromFloat(created))
                | _ => Js.Date.toISOString(Js.Date.make())
                };

              // let data = Js.Json.stringifyAny(state.config);
              // switch (data) {
              // | None => ()
              // | Some(data) =>
              //   downloadFile(title, data);
              // };
              downloadCanvas(
                title,
                Web.documentGetElementById("drawing")->Web.asCanvas,
              );
            }}
            onSave={scene => onSaveScene(scene)}
            key={
              switch (currentScene) {
              | None => "new-scene"
              | Some(scene) => scene.id
              }
            }
          />;
        }
        {Styles.spacer(8)}
        <Sidebar
          wasm
          update
          updateUi={ui => {
            // Js.log2("New ui", ui);
            wasm##update_ui(ui);
            dispatch(`Update((state.config, ui)));
          }}
          config={state.config}
          ui={state.ui}
        />
      </div>
    </div>;
  };
};

module App = {
  [@react.component]
  let make = (~wasm: Rust.wasm) => {
    let keys = Hooks.useLoading(ScenePicker.getSceneInfo);

    switch (keys) {
    | None => <div> {React.string("Loading")} </div>
    | Some(directory) => <Inner wasm directory blank={wasm##blank_config()} />
    };
  };
};

module CanvasTest = {
  [@react.component]
  let make = (~wasm) => {
    let r = React.useRef(Js.Nullable.null);
    React.useEffect1(
      () => {
        switch (React.Ref.current(r)->Js.Nullable.toOption) {
        | None => ()
        | Some(canvas) => wasm##test_run(canvas->Web.asCanvas)
        };
        None;
      },
      [|r|],
    );
    <canvas width="500" height="500" ref={ReactDOMRe.Ref.domRef(r)} />;
  };
};

Rust.withModule(wasm
  // wasm##run();
  // let config = wasm##save();
  =>
    ReactDOMRe.renderToElementWithId(
      <App wasm />,
      // <CanvasTest wasm />,
      // <WallEditor.TriangleTester />,
      "reason-root",
      // Js.log2("Config we got", config);
    )
  );