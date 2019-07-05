[@ocaml.warning "-34-39"];
module Types1 = {
  type _Belt_MapString__t('value) = Belt_MapString.t('value)
  and _Belt_SetString__t = Belt_SetString.t
  and _Types__directory = {
    scenes: _Belt_MapString__t(_Types__scene),
    tags: _Belt_MapString__t(_Types__tag),
  }
  and _Types__scene = {
    id: string,
    modified: float,
    created: float,
    title: option(string),
    tags: _Belt_SetString__t,
    children: array(string),
    parent: option(string),
  }
  and _Types__tag =
    Types.tag = {
      id: string,
      color: string,
      title: string,
    };
};
module Types2 = {
  type _Belt_MapString__t('value) = Belt_MapString.t('value)
  and _Belt_SetString__t = Belt_SetString.t
  and _Types__directory =
    Types.directory = {
      scenes: _Belt_MapString__t(_Types__scene),
      tags: _Belt_MapString__t(_Types__tag),
    }
  and _Types__scene =
    Types.scene = {
      id: string,
      modified: float,
      created: float,
      title: option(string),
      tags: _Belt_SetString__t,
      children: array(string),
      parent: option(string),
      starred: bool,
    }
  and _Types__tag =
    Types.tag = {
      id: string,
      color: string,
      title: string,
    };
  let rec migrate_Belt_MapString____t:
    'value 'value_migrated.
    ('value => 'value_migrated, Types1._Belt_MapString__t('value)) =>
    _Belt_MapString__t('value_migrated)
   =
    (type value, type value_migrated) => (
      (_migrator_value, _input_data) =>
        TransformHelpers.migrate_Belt_MapString__t(
          _migrator_value,
          _input_data,
        ):
        (value => value_migrated, Types1._Belt_MapString__t(value)) =>
        _Belt_MapString__t(value_migrated)
    )
  and migrate_Belt_SetString____t:
    Types1._Belt_SetString__t => _Belt_SetString__t =
    _input_data => _input_data
  and migrate_Types____directory: Types1._Types__directory => _Types__directory =
    _input_data => {
      let _converted_scenes =
        migrate_Belt_MapString____t(
          arg => migrate_Types____scene(arg),
          _input_data.scenes,
        );
      let _converted_tags =
        migrate_Belt_MapString____t(
          arg => migrate_Types____tag(arg),
          _input_data.tags,
        );
      {tags: _converted_tags, scenes: _converted_scenes};
    }
  and migrate_Types____scene: Types1._Types__scene => _Types__scene =
    _input_data => {
      let _converted_id = _input_data.id;
      let _converted_modified = _input_data.modified;
      let _converted_created = _input_data.created;
      let _converted_title =
        switch (_input_data.title) {
        | None => None
        | Some(_item) => Some(_item)
        };
      let _converted_tags = migrate_Belt_SetString____t(_input_data.tags);
      let _converted_children =
        _input_data.children |> Array.map(_item => _item);
      let _converted_parent =
        switch (_input_data.parent) {
        | None => None
        | Some(_item) => Some(_item)
        };
      let _converted_starred =
        (_ => false: Types1._Types__scene => bool)(_input_data);
      {
        starred: _converted_starred,
        parent: _converted_parent,
        children: _converted_children,
        tags: _converted_tags,
        title: _converted_title,
        created: _converted_created,
        modified: _converted_modified,
        id: _converted_id,
      };
    }
  and migrate_Types____tag: Types1._Types__tag => _Types__tag =
    _input_data => _input_data;
};
let currentVersion = 2;
type target = Js.Json.t;
let schemaPropertyName = "$schemaVersion";
type result('a, 'b) = Belt.Result.t('a, 'b) = | Ok('a) | Error('b);
module Version1 = {
  open Types1;
  let rec deserialize_Belt_MapString____t:
    type arg0.
      (target => result(arg0, list(string)), target) =>
      result(_Belt_MapString__t(arg0), list(string)) =
    valueTransformer =>
      TypeHelpers.deserialize_Belt_MapString____t(valueTransformer)
  and deserialize_Belt_SetString____t:
    target => result(_Belt_SetString__t, list(string)) = TypeHelpers.deserialize_Belt_SetString____t
  and deserialize_Types____directory:
    target => result(_Types__directory, list(string)) =
    record =>
      switch (Js.Json.classify(record)) {
      | JSONObject(dict) =>
        let inner = attr_tags => {
          let inner = attr_scenes =>
            Ok({scenes: attr_scenes, tags: attr_tags}: _Types__directory);
          switch (Js.Dict.get(dict, "scenes")) {
          | None => Belt.Result.Error(["No attribute 'scenes'"])
          | Some(json) =>
            switch (
              (deserialize_Belt_MapString____t(deserialize_Types____scene))(
                json,
              )
            ) {
            | Belt.Result.Error(error) =>
              Belt.Result.Error(["attribute 'scenes'", ...error])
            | Ok(data) => inner(data)
            }
          };
        };
        switch (Js.Dict.get(dict, "tags")) {
        | None => Belt.Result.Error(["No attribute 'tags'"])
        | Some(json) =>
          switch (
            (deserialize_Belt_MapString____t(deserialize_Types____tag))(json)
          ) {
          | Belt.Result.Error(error) =>
            Belt.Result.Error(["attribute 'tags'", ...error])
          | Ok(data) => inner(data)
          }
        };
      | _ => Belt.Result.Error(["Expected an object"])
      }
  and deserialize_Types____scene:
    target => result(_Types__scene, list(string)) =
    record =>
      switch (Js.Json.classify(record)) {
      | JSONObject(dict) =>
        let inner = attr_parent => {
          let inner = attr_children => {
            let inner = attr_tags => {
              let inner = attr_title => {
                let inner = attr_created => {
                  let inner = attr_modified => {
                    let inner = attr_id =>
                      Ok(
                        {
                          id: attr_id,
                          modified: attr_modified,
                          created: attr_created,
                          title: attr_title,
                          tags: attr_tags,
                          children: attr_children,
                          parent: attr_parent,
                        }: _Types__scene,
                      );
                    switch (Js.Dict.get(dict, "id")) {
                    | None => Belt.Result.Error(["No attribute 'id'"])
                    | Some(json) =>
                      switch (
                        (
                          string =>
                            switch (Js.Json.classify(string)) {
                            | JSONString(string) => Belt.Result.Ok(string)
                            | _ => Error(["expected a string"])
                            }
                        )(
                          json,
                        )
                      ) {
                      | Belt.Result.Error(error) =>
                        Belt.Result.Error(["attribute 'id'", ...error])
                      | Ok(data) => inner(data)
                      }
                    };
                  };
                  switch (Js.Dict.get(dict, "modified")) {
                  | None => Belt.Result.Error(["No attribute 'modified'"])
                  | Some(json) =>
                    switch (
                      (
                        number =>
                          switch (Js.Json.classify(number)) {
                          | JSONNumber(number) => Belt.Result.Ok(number)
                          | _ => Error(["Expected a float"])
                          }
                      )(
                        json,
                      )
                    ) {
                    | Belt.Result.Error(error) =>
                      Belt.Result.Error(["attribute 'modified'", ...error])
                    | Ok(data) => inner(data)
                    }
                  };
                };
                switch (Js.Dict.get(dict, "created")) {
                | None => Belt.Result.Error(["No attribute 'created'"])
                | Some(json) =>
                  switch (
                    (
                      number =>
                        switch (Js.Json.classify(number)) {
                        | JSONNumber(number) => Belt.Result.Ok(number)
                        | _ => Error(["Expected a float"])
                        }
                    )(
                      json,
                    )
                  ) {
                  | Belt.Result.Error(error) =>
                    Belt.Result.Error(["attribute 'created'", ...error])
                  | Ok(data) => inner(data)
                  }
                };
              };
              switch (Js.Dict.get(dict, "title")) {
              | None => inner(None)
              | Some(json) =>
                switch (
                  (
                    (
                      (transformer, option) =>
                        switch (Js.Json.classify(option)) {
                        | JSONNull => Belt.Result.Ok(None)
                        | _ =>
                          switch (transformer(option)) {
                          | Belt.Result.Error(error) =>
                            Belt.Result.Error(["optional value", ...error])
                          | Ok(value) => Ok(Some(value))
                          }
                        }
                    )(
                      string =>
                      switch (Js.Json.classify(string)) {
                      | JSONString(string) => Belt.Result.Ok(string)
                      | _ => Error(["expected a string"])
                      }
                    )
                  )(
                    json,
                  )
                ) {
                | Belt.Result.Error(error) =>
                  Belt.Result.Error(["attribute 'title'", ...error])
                | Ok(data) => inner(data)
                }
              };
            };
            switch (Js.Dict.get(dict, "tags")) {
            | None => Belt.Result.Error(["No attribute 'tags'"])
            | Some(json) =>
              switch (deserialize_Belt_SetString____t(json)) {
              | Belt.Result.Error(error) =>
                Belt.Result.Error(["attribute 'tags'", ...error])
              | Ok(data) => inner(data)
              }
            };
          };
          switch (Js.Dict.get(dict, "children")) {
          | None => Belt.Result.Error(["No attribute 'children'"])
          | Some(json) =>
            switch (
              (
                (
                  (transformer, array) =>
                    switch (Js.Json.classify(array)) {
                    | JSONArray(items) =>
                      let rec loop = (i, collected, items) =>
                        switch (items) {
                        | [] => Belt.Result.Ok(Belt.List.reverse(collected))
                        | [one, ...rest] =>
                          switch (transformer(one)) {
                          | Belt.Result.Error(error) =>
                            Belt.Result.Error([
                              "list element " ++ string_of_int(i),
                              ...error,
                            ])
                          | Ok(value) =>
                            loop(i + 1, [value, ...collected], rest)
                          }
                        };
                      switch (loop(0, [], Belt.List.fromArray(items))) {
                      | Belt.Result.Error(error) => Belt.Result.Error(error)
                      | Ok(value) => Ok(Belt.List.toArray(value))
                      };
                    | _ => Belt.Result.Error(["expected an array"])
                    }
                )(
                  string =>
                  switch (Js.Json.classify(string)) {
                  | JSONString(string) => Belt.Result.Ok(string)
                  | _ => Error(["expected a string"])
                  }
                )
              )(
                json,
              )
            ) {
            | Belt.Result.Error(error) =>
              Belt.Result.Error(["attribute 'children'", ...error])
            | Ok(data) => inner(data)
            }
          };
        };
        switch (Js.Dict.get(dict, "parent")) {
        | None => inner(None)
        | Some(json) =>
          switch (
            (
              (
                (transformer, option) =>
                  switch (Js.Json.classify(option)) {
                  | JSONNull => Belt.Result.Ok(None)
                  | _ =>
                    switch (transformer(option)) {
                    | Belt.Result.Error(error) =>
                      Belt.Result.Error(["optional value", ...error])
                    | Ok(value) => Ok(Some(value))
                    }
                  }
              )(
                string =>
                switch (Js.Json.classify(string)) {
                | JSONString(string) => Belt.Result.Ok(string)
                | _ => Error(["expected a string"])
                }
              )
            )(
              json,
            )
          ) {
          | Belt.Result.Error(error) =>
            Belt.Result.Error(["attribute 'parent'", ...error])
          | Ok(data) => inner(data)
          }
        };
      | _ => Belt.Result.Error(["Expected an object"])
      }
  and deserialize_Types____tag: target => result(_Types__tag, list(string)) =
    record =>
      switch (Js.Json.classify(record)) {
      | JSONObject(dict) =>
        let inner = attr_title => {
          let inner = attr_color => {
            let inner = attr_id =>
              Ok(
                {id: attr_id, color: attr_color, title: attr_title}: _Types__tag,
              );
            switch (Js.Dict.get(dict, "id")) {
            | None => Belt.Result.Error(["No attribute 'id'"])
            | Some(json) =>
              switch (
                (
                  string =>
                    switch (Js.Json.classify(string)) {
                    | JSONString(string) => Belt.Result.Ok(string)
                    | _ => Error(["expected a string"])
                    }
                )(
                  json,
                )
              ) {
              | Belt.Result.Error(error) =>
                Belt.Result.Error(["attribute 'id'", ...error])
              | Ok(data) => inner(data)
              }
            };
          };
          switch (Js.Dict.get(dict, "color")) {
          | None => Belt.Result.Error(["No attribute 'color'"])
          | Some(json) =>
            switch (
              (
                string =>
                  switch (Js.Json.classify(string)) {
                  | JSONString(string) => Belt.Result.Ok(string)
                  | _ => Error(["expected a string"])
                  }
              )(
                json,
              )
            ) {
            | Belt.Result.Error(error) =>
              Belt.Result.Error(["attribute 'color'", ...error])
            | Ok(data) => inner(data)
            }
          };
        };
        switch (Js.Dict.get(dict, "title")) {
        | None => Belt.Result.Error(["No attribute 'title'"])
        | Some(json) =>
          switch (
            (
              string =>
                switch (Js.Json.classify(string)) {
                | JSONString(string) => Belt.Result.Ok(string)
                | _ => Error(["expected a string"])
                }
            )(
              json,
            )
          ) {
          | Belt.Result.Error(error) =>
            Belt.Result.Error(["attribute 'title'", ...error])
          | Ok(data) => inner(data)
          }
        };
      | _ => Belt.Result.Error(["Expected an object"])
      };
};
module Version2 = {
  open Types2;
  let rec deserialize_Belt_MapString____t:
    type arg0.
      (target => result(arg0, list(string)), target) =>
      result(_Belt_MapString__t(arg0), list(string)) = Version1.deserialize_Belt_MapString____t
  and deserialize_Belt_SetString____t:
    target => result(_Belt_SetString__t, list(string)) = Version1.deserialize_Belt_SetString____t
  and deserialize_Types____directory:
    target => result(_Types__directory, list(string)) =
    record =>
      switch (Js.Json.classify(record)) {
      | JSONObject(dict) =>
        let inner = attr_tags => {
          let inner = attr_scenes =>
            Ok({scenes: attr_scenes, tags: attr_tags}: _Types__directory);
          switch (Js.Dict.get(dict, "scenes")) {
          | None => Belt.Result.Error(["No attribute 'scenes'"])
          | Some(json) =>
            switch (
              (deserialize_Belt_MapString____t(deserialize_Types____scene))(
                json,
              )
            ) {
            | Belt.Result.Error(error) =>
              Belt.Result.Error(["attribute 'scenes'", ...error])
            | Ok(data) => inner(data)
            }
          };
        };
        switch (Js.Dict.get(dict, "tags")) {
        | None => Belt.Result.Error(["No attribute 'tags'"])
        | Some(json) =>
          switch (
            (deserialize_Belt_MapString____t(deserialize_Types____tag))(json)
          ) {
          | Belt.Result.Error(error) =>
            Belt.Result.Error(["attribute 'tags'", ...error])
          | Ok(data) => inner(data)
          }
        };
      | _ => Belt.Result.Error(["Expected an object"])
      }
  and deserialize_Types____scene:
    target => result(_Types__scene, list(string)) =
    record =>
      switch (Js.Json.classify(record)) {
      | JSONObject(dict) =>
        let inner = attr_starred => {
          let inner = attr_parent => {
            let inner = attr_children => {
              let inner = attr_tags => {
                let inner = attr_title => {
                  let inner = attr_created => {
                    let inner = attr_modified => {
                      let inner = attr_id =>
                        Ok(
                          {
                            id: attr_id,
                            modified: attr_modified,
                            created: attr_created,
                            title: attr_title,
                            tags: attr_tags,
                            children: attr_children,
                            parent: attr_parent,
                            starred: attr_starred,
                          }: _Types__scene,
                        );
                      switch (Js.Dict.get(dict, "id")) {
                      | None => Belt.Result.Error(["No attribute 'id'"])
                      | Some(json) =>
                        switch (
                          (
                            string =>
                              switch (Js.Json.classify(string)) {
                              | JSONString(string) => Belt.Result.Ok(string)
                              | _ => Error(["expected a string"])
                              }
                          )(
                            json,
                          )
                        ) {
                        | Belt.Result.Error(error) =>
                          Belt.Result.Error(["attribute 'id'", ...error])
                        | Ok(data) => inner(data)
                        }
                      };
                    };
                    switch (Js.Dict.get(dict, "modified")) {
                    | None => Belt.Result.Error(["No attribute 'modified'"])
                    | Some(json) =>
                      switch (
                        (
                          number =>
                            switch (Js.Json.classify(number)) {
                            | JSONNumber(number) => Belt.Result.Ok(number)
                            | _ => Error(["Expected a float"])
                            }
                        )(
                          json,
                        )
                      ) {
                      | Belt.Result.Error(error) =>
                        Belt.Result.Error(["attribute 'modified'", ...error])
                      | Ok(data) => inner(data)
                      }
                    };
                  };
                  switch (Js.Dict.get(dict, "created")) {
                  | None => Belt.Result.Error(["No attribute 'created'"])
                  | Some(json) =>
                    switch (
                      (
                        number =>
                          switch (Js.Json.classify(number)) {
                          | JSONNumber(number) => Belt.Result.Ok(number)
                          | _ => Error(["Expected a float"])
                          }
                      )(
                        json,
                      )
                    ) {
                    | Belt.Result.Error(error) =>
                      Belt.Result.Error(["attribute 'created'", ...error])
                    | Ok(data) => inner(data)
                    }
                  };
                };
                switch (Js.Dict.get(dict, "title")) {
                | None => inner(None)
                | Some(json) =>
                  switch (
                    (
                      (
                        (transformer, option) =>
                          switch (Js.Json.classify(option)) {
                          | JSONNull => Belt.Result.Ok(None)
                          | _ =>
                            switch (transformer(option)) {
                            | Belt.Result.Error(error) =>
                              Belt.Result.Error(["optional value", ...error])
                            | Ok(value) => Ok(Some(value))
                            }
                          }
                      )(
                        string =>
                        switch (Js.Json.classify(string)) {
                        | JSONString(string) => Belt.Result.Ok(string)
                        | _ => Error(["expected a string"])
                        }
                      )
                    )(
                      json,
                    )
                  ) {
                  | Belt.Result.Error(error) =>
                    Belt.Result.Error(["attribute 'title'", ...error])
                  | Ok(data) => inner(data)
                  }
                };
              };
              switch (Js.Dict.get(dict, "tags")) {
              | None => Belt.Result.Error(["No attribute 'tags'"])
              | Some(json) =>
                switch (deserialize_Belt_SetString____t(json)) {
                | Belt.Result.Error(error) =>
                  Belt.Result.Error(["attribute 'tags'", ...error])
                | Ok(data) => inner(data)
                }
              };
            };
            switch (Js.Dict.get(dict, "children")) {
            | None => Belt.Result.Error(["No attribute 'children'"])
            | Some(json) =>
              switch (
                (
                  (
                    (transformer, array) =>
                      switch (Js.Json.classify(array)) {
                      | JSONArray(items) =>
                        let rec loop = (i, collected, items) =>
                          switch (items) {
                          | [] =>
                            Belt.Result.Ok(Belt.List.reverse(collected))
                          | [one, ...rest] =>
                            switch (transformer(one)) {
                            | Belt.Result.Error(error) =>
                              Belt.Result.Error([
                                "list element " ++ string_of_int(i),
                                ...error,
                              ])
                            | Ok(value) =>
                              loop(i + 1, [value, ...collected], rest)
                            }
                          };
                        switch (loop(0, [], Belt.List.fromArray(items))) {
                        | Belt.Result.Error(error) =>
                          Belt.Result.Error(error)
                        | Ok(value) => Ok(Belt.List.toArray(value))
                        };
                      | _ => Belt.Result.Error(["expected an array"])
                      }
                  )(
                    string =>
                    switch (Js.Json.classify(string)) {
                    | JSONString(string) => Belt.Result.Ok(string)
                    | _ => Error(["expected a string"])
                    }
                  )
                )(
                  json,
                )
              ) {
              | Belt.Result.Error(error) =>
                Belt.Result.Error(["attribute 'children'", ...error])
              | Ok(data) => inner(data)
              }
            };
          };
          switch (Js.Dict.get(dict, "parent")) {
          | None => inner(None)
          | Some(json) =>
            switch (
              (
                (
                  (transformer, option) =>
                    switch (Js.Json.classify(option)) {
                    | JSONNull => Belt.Result.Ok(None)
                    | _ =>
                      switch (transformer(option)) {
                      | Belt.Result.Error(error) =>
                        Belt.Result.Error(["optional value", ...error])
                      | Ok(value) => Ok(Some(value))
                      }
                    }
                )(
                  string =>
                  switch (Js.Json.classify(string)) {
                  | JSONString(string) => Belt.Result.Ok(string)
                  | _ => Error(["expected a string"])
                  }
                )
              )(
                json,
              )
            ) {
            | Belt.Result.Error(error) =>
              Belt.Result.Error(["attribute 'parent'", ...error])
            | Ok(data) => inner(data)
            }
          };
        };
        switch (Js.Dict.get(dict, "starred")) {
        | None => Belt.Result.Error(["No attribute 'starred'"])
        | Some(json) =>
          switch (
            (
              bool =>
                switch (Js.Json.classify(bool)) {
                | JSONTrue => Belt.Result.Ok(true)
                | JSONFalse => Belt.Result.Ok(false)
                | _ => Belt.Result.Error(["Expected a bool"])
                }
            )(
              json,
            )
          ) {
          | Belt.Result.Error(error) =>
            Belt.Result.Error(["attribute 'starred'", ...error])
          | Ok(data) => inner(data)
          }
        };
      | _ => Belt.Result.Error(["Expected an object"])
      }
  and deserialize_Types____tag: target => result(_Types__tag, list(string)) = Version1.deserialize_Types____tag
  and serialize_Belt_MapString____t:
    'arg0.
    ('arg0 => target, _Belt_MapString__t('arg0)) => target
   =
    valueTransformer =>
      TypeHelpers.serialize_Belt_MapString____t(valueTransformer)
  and serialize_Belt_SetString____t: _Belt_SetString__t => target = TypeHelpers.serialize_Belt_SetString____t
  and serialize_Types____directory: _Types__directory => target =
    record =>
      Js.Json.object_(
        Js.Dict.fromArray([|
          (
            "scenes",
            (serialize_Belt_MapString____t(serialize_Types____scene))(
              record.scenes,
            ),
          ),
          (
            "tags",
            (serialize_Belt_MapString____t(serialize_Types____tag))(
              record.tags,
            ),
          ),
        |]),
      )
  and serialize_Types____scene: _Types__scene => target =
    record =>
      Js.Json.object_(
        Js.Dict.fromArray([|
          ("id", Js.Json.string(record.id)),
          ("modified", Js.Json.number(record.modified)),
          ("created", Js.Json.number(record.created)),
          (
            "title",
            (
              (
                transformer =>
                  fun
                  | Some(inner) => transformer(inner)
                  | None => Js.Json.null
              )(
                Js.Json.string,
              )
            )(
              record.title,
            ),
          ),
          ("tags", serialize_Belt_SetString____t(record.tags)),
          (
            "children",
            (
              (
                (transformer, array) =>
                  Js.Json.array((Belt.Array.map(array))(transformer))
              )(
                Js.Json.string,
              )
            )(
              record.children,
            ),
          ),
          (
            "parent",
            (
              (
                transformer =>
                  fun
                  | Some(inner) => transformer(inner)
                  | None => Js.Json.null
              )(
                Js.Json.string,
              )
            )(
              record.parent,
            ),
          ),
          ("starred", Js.Json.boolean(record.starred)),
        |]),
      )
  and serialize_Types____tag: _Types__tag => target =
    record =>
      Js.Json.object_(
        Js.Dict.fromArray([|
          ("id", Js.Json.string(record.id)),
          ("color", Js.Json.string(record.color)),
          ("title", Js.Json.string(record.title)),
        |]),
      );
};
module Current = Version2;
let parseVersion = json =>
  switch (Js.Json.classify(json)) {
  | JSONObject(dict) =>
    switch (Js.Dict.get(dict, schemaPropertyName)) {
    | Some(schemaVersion) =>
      switch (Js.Json.classify(schemaVersion)) {
      | JSONNumber(version) =>
        [@implicit_arity] Belt.Result.Ok(int_of_float(version), json)
      | _ => Belt.Result.Error("Invalid " ++ schemaPropertyName)
      }
    | None => Belt.Result.Error("No " ++ schemaPropertyName ++ " present")
    }
  | JSONArray([|version, payload|]) =>
    switch (Js.Json.classify(version)) {
    | JSONNumber(version) =>
      [@implicit_arity] Belt.Result.Ok(int_of_float(version), payload)
    | _ => Belt.Result.Error("Invalid wrapped version")
    }
  | _ => Belt.Result.Error("Must have a schema version")
  };
let wrapWithVersion = (version, payload) =>
  switch (Js.Json.classify(payload)) {
  | JSONObject(dict) =>
    Js.Dict.set(
      dict,
      schemaPropertyName,
      Js.Json.number(float_of_int(version)),
    );
    Js.Json.object_(dict);
  | _ => Js.Json.array([|Js.Json.number(float_of_int(version)), payload|])
  };
let serializeDirectory = data =>
  wrapWithVersion(
    currentVersion,
    Version2.serialize_Types____directory(data),
  )
and deserializeDirectory = data =>
  switch (parseVersion(data)) {
  | Error(err) => Error([err])
  | [@implicit_arity] Ok(version, data) =>
    switch (version) {
    | 2 =>
      switch (Version2.deserialize_Types____directory(data)) {
      | Error(error) => Error(error)
      | Ok(data) => Ok(data)
      }
    | 1 =>
      switch (Version1.deserialize_Types____directory(data)) {
      | Error(error) => Error(error)
      | Ok(data) =>
        let data = Types2.migrate_Types____directory(data);
        Ok(data);
      }
    | _ => Error(["Unexpected version " ++ string_of_int(version)])
    }
  };
module Modules = {
  module Directory = {
    type t = Types2._Types__directory;
    let serialize = serializeDirectory;
    let deserialize = deserializeDirectory;
  };
};
