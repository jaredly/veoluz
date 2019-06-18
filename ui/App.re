
let x = 10;

[@bs.module "localforage"]
external getItem: string => Js.Promise.t('a) = "";

[@bs.module "localforage"]
external setItem: (string, 'a) => Js.Promise.t(unit) = "";

[@bs.module "localforage"]
external keys: unit => Js.Promise.t(array(string)) = "";

module App = {

  let getKeys = () => keys();

  [@react.component]
  let make = (~config: Rust.config) => {
    let keys = Hooks.useLoading(getKeys);

    switch keys {
      | None => <div>{React.string("Loading")}</div>
      | Some(keys) => <div>
        {React.array(
          keys->Belt.Array.map(key => {
            <div>
              {React.string(key)}
            </div>
          })
        )}
      </div>
    }

  }
}

Rust.withModule(wasm => {
  wasm##run();
  let config = wasm##save();
  ReactDOMRe.renderToElementWithId(<App config />, "reason-root");
  Js.log2("Config we got", config);
});
