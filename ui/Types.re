
type scene = {
  id: string,
  tags: Belt.Set.String.t,
  children: array(string),
  parent: string,
};

type directory = array(scene);
