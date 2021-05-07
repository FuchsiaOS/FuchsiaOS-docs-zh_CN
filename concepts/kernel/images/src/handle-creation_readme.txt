# Export images from source DOT files

zircon/docs/handles.md contains images related to handle creation:
handle-creation1.png to handle-creation4.png.

The sources for these images are specified using the DOT language (more
information at https://www.graphviz.org/doc/info/lang.html). You can find the
sources in zircon/docs/handles.md.

To render a png from the source DOT, do either of the following:

*   Save the graph information to a temporary file (e.g., tmp.dot). For example:

    digraph Q {
      node [shape=record];
      nd_1 [label = "Hello"];
      nd_2 [label = "World", style = invis];

      subgraph cluster_Example1 {
        label = "Example1";
        nd_1
      }

      subgraph cluster_Example2 {
        label = "Example2";
        style=filled;
        nd_2;
      }
    }

    # Then run dot -Tpng tmp.dot -o example.png if Graphviz is installed.

*   Paste the graph information into an online editor (e.g.,
    http://www.webgraphviz.com/) and then save the resulting image.
