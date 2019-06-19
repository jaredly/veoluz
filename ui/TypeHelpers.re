open Belt.Result;

let deserialize_Belt_SetString____t = json =>
  switch (Js.Json.classify(json)) {
  | Js.Json.JSONArray(items) =>
    items->Belt.Array.reduce(
      Belt.Result.Ok(Belt.Set.String.empty), (set, item) =>
      switch (set, Js.Json.classify(item)) {
      | (Ok(set), Js.Json.JSONString(string)) =>
        Ok(set->Belt.Set.String.add(string))
      | (Error(e), _) => Error(e)
      | (_, _) => Error(["Expected a string in the set array"])
      }
    )
  | _ => Belt.Result.Error(["Expected an array"])
  };

let serialize_Belt_SetString____t = set =>
  set->Belt.Set.String.toArray->Js.Json.stringArray;

let deserialize_Belt_MapString____t = (converter, json) =>
  switch (Js.Json.classify(json)) {
  | Js.Json.JSONObject(dict) =>
    dict
    ->Js.Dict.entries
    ->Belt.Array.reduce(Ok(Belt.Map.String.empty), (map, (key, value)) =>
        switch (map) {
        | Error(_) => map
        | Ok(map) =>
          switch (converter(value)) {
          | Error(m) => Error(m)
          | Ok(v) => Ok(map->Belt.Map.String.set(key, v))
          }
        }
      )
  | _ => Error(["Expected an object"])
  };

let serialize_Belt_MapString____t = (converter, map) =>
  map
  ->Belt.Map.String.toArray
  ->Belt.Array.map(((k, v)) => (k, converter(v)))
  ->Js.Dict.fromArray
  ->Js.Json.object_;