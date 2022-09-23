#! Hello World example

(import std.io)
(import std.string (removeAll) string)

(defsig main (Fun IO IO))
(defun main io (println io (string.removeAll '\'' "Hel'lo World!")))

(main stdIO)
