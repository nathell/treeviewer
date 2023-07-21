#!/usr/bin/env bb
(require '[babashka.process :refer [shell process exec]])
(require '[babashka.fs :as fs])
(require '[clojure.java.io :as io])
(require '[clojure.string :as str])

(def root-dir (fs/parent (fs/parent (fs/normalize *file*))))

(defn init-tmpdir []
  (shell {:out :string, :err :string, :dir "/tmp"} "git clone" root-dir))

(defn clean-tmpdir []
  (shell "rm -rf /tmp/treeviewer"))

(defn try-build [tag]
  (shell {:out :string, :err :string, :dir "/tmp/treeviewer"} "git checkout" tag)
  (shell {:out :string, :err :string, :dir "/tmp/treeviewer", :continue true} "cargo build"))

(shell "mkdir -p diffs")
(init-tmpdir)
(let [tags (-> (shell {:out :string} "git" "tag")
               :out
               (str/split #"\n"))]
  (spit "diffs/build-logs.txt"
        (with-out-str
          (doseq [[prev-tag tag] (partition 2 1 tags)]
            (let [build (try-build tag)
                  ok? (zero? (:exit build))
                  diff (shell {:out :string} "git diff" prev-tag tag)]
              (println "======")
              (printf "%s: %s\n" tag (if ok? "SUCCESS" "FAILURE"))
              (println "DIFF:")
              (println (:out diff))
              (when-not ok?
                (println "ERRORS:")
                (println (:out build) (:err build))))))))
(clean-tmpdir)
