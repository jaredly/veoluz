

open Belt.Result;

let deserialize_Belt_SetString____t = json => switch (Js.Json.classify(json)) {
  | Js.Json.JSONArray(items) => items->Belt.Array.reduce(Belt.Result.Ok(Belt.Set.String.empty), (set, item) => {
    switch (set, Js.Json.classify(item)) {
      | (Ok(set), Js.Json.JSONString(string)) => Ok(set->Belt.Set.String.add(string))
      | (Error(e), _) => Error(e)
      | (_, _) => Error(["Expected a string in the set array"])
    }
  })
  | _ => Belt.Result.Error(["Expected an array"])
}

let serialize_Belt_SetString____t = set => set->Belt.Set.String.toArray->Js.Json.stringArray;