(use-package-modules rust entr)

(packages->manifest
 (list rust 
       rust:cargo
       entr))
