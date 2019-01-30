(define (add-interval x y)
  (make-interval (+ (lower-bound x) (lower-bound y))
                 (+ (upper-bound x) (upper-bound y))))

(define (sub-interval x y)
  (make-interval (- (lower-bound x) (upper-bound y))
                 (- (upper-bound x) (lower-bound y))))

(define (make-interval a b) (cons a b))

(define (lower-bound x) (car x))

(define (upper-bound x) (cdr x))

(define (width x) (/ (- (upper-bound x) (lower-bound x)) 2))

(define (add-width x-width y-width) (+ x-width y-width))

(define (add-width2 x y) (width (add-interval x y)))

(define (sub-width x-width y-width) (+ x-width y-width))

(define (sub-width2 x y) (width (sub-interval x y)))
