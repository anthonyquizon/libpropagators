(use-modules (srfi srfi-1))

(define (pairwise-union nogoods1 nogoods2)
  (apply
    append
    (map (lambda (nogood1)
           (map (lambda (nogood2) 
                            (lset-union eq? nogood1 nogood2))
                          nogoods2))
           nogoods1)))

(display (pairwise-union '((1 2 ) (3 4)) '((a b) (c d))))
