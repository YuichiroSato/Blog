(define (tree-map f tree)
  (if (pair? tree)
      (map (lambda (t) (tree-map f t)) tree)
      (f tree)))

(define (square-tree tree)
  (tree-map square tree))
