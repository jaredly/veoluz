
type scene = {
  id: string,
  modified: float,
  created: float,
  tags: Belt.Set.String.t,
  children: array(string),
  parent: option(string),
};

type tag = {
  id: string,
  color: string,
  title: string,
};

type directory = {
  scenes: Belt.Map.String.t(scene),
  tags: Belt.Map.String.t(tag),
};
