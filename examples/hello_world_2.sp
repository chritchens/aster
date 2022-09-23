#! Hello World example

(import std.io)
(import std.string (removeAll) string)

(defsig main (Fun Empty IO))
(defun main e (println (string.removeAll '\'' "Hel'lo World!") e))

(main ())
