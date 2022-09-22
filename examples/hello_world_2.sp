#! Hello World example

(import std.io)
(import std.string (removeAll) string)

(println (string.removeAll '\'' "Hel'lo World!"))
