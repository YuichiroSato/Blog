(use gl)
(use gl.glut)

(define (main args)
  (glut-init args)
  (glut-init-display-mode GLUT_RGBA)
  (glut-create-window "painter sample")
  (do-display (compose ((segments->painter
                          (list s1 s2 s3 s4))
                          frame1)))
  (init)
  (glut-main-loop))

(define (do-display f)
  (glut-display-func f))

(define (compose procs)
  (define (iter fs)
    (cond ((null? fs) (lambda () '()))
          ((= (length fs) 1) ((car fs)))
          (else (lambda () (begin ((car fs)) (iter (cdr fs)))))))
  (iter procs))
        
(define (draw-line start end)
  (lambda ()
    (begin
      (gl-color 1.0 0.0 0.0)
      (gl-begin GL_LINES)
      (gl-vertex (get-x start) (get-y start))
      (gl-vertex (get-x end) (get-y end))
      (gl-end)
      (gl-flush))))

(define (init)
  (gl-clear-color 0.0 0.0 0.0 1.0))

(define (make-vect x y)
  (list x y))

(define (get-x v) (car v))

(define (get-y v) (cadr v))

(define (add-vect v w)
  (map + v w))

(define (sub-vect v w)
  (map - v w))

(define (scale-vect a v)
  (map (lambda (x) (* a x)) v))

(define (make-frame origin edge1 edge2)
  (list origin edge1 edge2))

(define (edge1-frame frame)
  (cadr frame))

(define (edge2-frame frame)
  (caddr frame))

(define (origin-frame frame)
  (car frame))

(define (make-segment start start-to-end)
  (list start start-to-end))

(define (start-segment segment)
  (car segment))

(define (end-segment segment)
  (cadr segment))

(define (frame-coord-map frame)
  (lambda (v)
    (add-vect
      (origin-frame frame)
      (add-vect (scale-vect (get-x v)
                            (edge1-frame frame))
                (scale-vect (get-y v)
                            (edge2-frame frame))))))

(define (segments->painter segment-list)
  (lambda (frame)
    (map
      (lambda (segment)
        (draw-line
          ((frame-coord-map frame) (start-segment segment))
          ((frame-coord-map frame) (end-segment segment))))
    segment-list)))

(use slib)
(require 'trace)
(trace draw-line)

(define frame1 (make-frame (make-vect 0.0 0.0) (make-vect 0.3 0.1) (make-vect 0.1 0.2)))

(define segment1 (make-segment (make-vect 0.5 0.5) (make-vect 1.0 1.0)))

(define segment2 (make-segment (make-vect 0.2 0.2) (make-vect -1.0 -1.0)))

(define s1 (make-segment (make-vect 0 0) (make-vect 0 1)))

(define s2 (make-segment (make-vect 0 0) (make-vect 1 0)))

(define s3 (make-segment (make-vect 1 1) (make-vect 0 1)))

(define s4 (make-segment (make-vect 1 1) (make-vect 1 0)))
