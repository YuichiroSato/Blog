(define (make-vect x y)
  (list x y))

(define (add-vect v w)
  (map + v w))

(define (sub-vect v w)
  (map - v w))

(define (scale-vect a v)
  (map (lambda (x) (* a x)) v))
