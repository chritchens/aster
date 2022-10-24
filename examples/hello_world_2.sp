#! Hello World example

(import std.io)
(import std.string string (prod removeAll))

(def sig main (Fun IO IO))
(def fun main io (println io (string.removeAll '\'' "Hel'lo World!")))

(main stdIO)
