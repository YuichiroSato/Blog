(define (mul-interval-app x y)
  (let ((c1 (center x))
        (c2 (center y)))
    (make-center-percent (* c1 c2) (/ (+ (* c1 (percent x))
                                         (* c2 (percent y)))
                                      100.0))))

(define (make-center-percent c p)
  (make-center-width c (* c (/ p 100.0))))

(define (make-center-width c w)
  (make-interval (- c w) (+ c w)))

(define (center i)
  (/ (+ (lower-bound i) (upper-bound i)) 2))

(define (width i)
  (/ (- (upper-bound i) (lower-bound i)) 2))

(define (percent i)
  (* (/ (width i) (center i)) 100.0))

(define (make-interval a b) (cons a b))

(define (lower-bound x) (car x))

(define (upper-bound x) (cdr x))
