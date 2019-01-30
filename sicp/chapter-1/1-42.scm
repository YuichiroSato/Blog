(define (compose f g)
  (lambda (x) (f (g x))))

(define (square x)
  (* x x))

(define (inc x)
  (+ x 1))

(define (main args)
  (display "((compose square inc) 6)\n")
  (format #t "~d\n" ((compose square inc) 6)))
