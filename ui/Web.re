
[@bs.module "querystring"] external stringify: Js.t('a) => string = "";

[@bs.module "url"] external urlParse: (string, bool) => {. "query": Js.nullable({. "error": Js.nullable(string), "code": Js.nullable(string)})} = "parse";

module LocalForage = {

  [@bs.module "localforage"]
  external getItem: string => Js.Promise.t(Js.nullable('a)) = "";

  [@bs.module "localforage"]
  external setItem: (string, 'a) => Js.Promise.t(unit) = "";

  [@bs.module "localforage"]
  external keys: unit => Js.Promise.t(array(string)) = "";

}

type blob;
type canvas;

[@bs.send] external toBlob: (canvas, blob => unit) => unit = "";
external asCanvas: Dom.element => canvas = "%identity";

[@bs.val] [@bs.scope "document"]
external documentGetElementById: string => Dom.element = "getElementById";

type headers;
[@bs.send] external get: (headers, string) => string = "";

type response = {."headers": headers, "status": int};

[@bs.val] external fetch: (string, 'config) => Js.Promise.t(response) = "";
[@bs.send] external json: response => Js.Promise.t('a) = "";

type document;
[@bs.val] external documentEl: document = "document";
[@bs.get] external body: document => Dom.element = "";
[@bs.send] external appendChild: (Dom.element, Dom.element) => unit = "";

type window;
[@bs.val] external window: window = "";
[@bs.val] external document: Dom.element = "";
[@bs.send] external createElement: (Dom.element, string) => Dom.element = "";
[@bs.send] external addEventListener: (window, string, ('event) => unit, bool) => unit = "";
[@bs.send] external removeEventListener: (window, string, ('event) => unit, bool) => unit = "";
[@bs.send] external getElementById: (Dom.element, string) => option(Dom.element) = "";
[@bs.get] external offsetParent: Dom.element => Dom.element = "";
type rect = {
  .
  "top": float,
  "height": float,
  "width": float,
  "left": float,
  "bottom": float,
  "right": float,
};
[@bs.send] external getBoundingClientRect: Dom.element => rect = "";

module Location = {
  type t;
  [@bs.val] external location: t = "";
  [@bs.get] external hash: t => string = "";
  [@bs.set] external setHash: (t, string) => unit = "hash";
  let hash = () => location->hash;
  let setHash = hash => location->setHash(hash);
  let addHashListener = (fn) => {
    let listener = () => fn(hash());
    window->addEventListener("hashchange", listener, false);
    () => window->removeEventListener("hashchange", listener, false);
  };
};
