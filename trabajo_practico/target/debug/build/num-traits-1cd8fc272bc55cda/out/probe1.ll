; ModuleID = 'probe1.5dfbba22-cgu.0'
source_filename = "probe1.5dfbba22-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@alloc6 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"\00\00\00\00\00\00\F0?" }>, align 8
@alloc8 = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"\00\00\00\00\00\00\00@" }>, align 8

; core::f64::<impl f64>::total_cmp
; Function Attrs: inlinehint nonlazybind uwtable
define internal i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17hf2bb722132daa76dE"(ptr align 8 %self, ptr align 8 %other) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %1 = alloca i64, align 8
  %_26 = alloca double, align 8
  %_22 = alloca double, align 8
  %right = alloca i64, align 8
  %left = alloca i64, align 8
  %2 = alloca i8, align 1
  %self1 = load double, ptr %self, align 8, !noundef !2
  store double %self1, ptr %_22, align 8
  %rt = load double, ptr %_22, align 8, !noundef !2
  %3 = bitcast double %rt to i64
  store i64 %3, ptr %1, align 8
  %_4 = load i64, ptr %1, align 8, !noundef !2
  store i64 %_4, ptr %left, align 8
  %self2 = load double, ptr %other, align 8, !noundef !2
  store double %self2, ptr %_26, align 8
  %rt3 = load double, ptr %_26, align 8, !noundef !2
  %4 = bitcast double %rt3 to i64
  store i64 %4, ptr %0, align 8
  %_7 = load i64, ptr %0, align 8, !noundef !2
  store i64 %_7, ptr %right, align 8
  %_13 = load i64, ptr %left, align 8, !noundef !2
  %_12 = ashr i64 %_13, 63
  %_10 = lshr i64 %_12, 1
  %5 = load i64, ptr %left, align 8, !noundef !2
  %6 = xor i64 %5, %_10
  store i64 %6, ptr %left, align 8
  %_18 = load i64, ptr %right, align 8, !noundef !2
  %_17 = ashr i64 %_18, 63
  %_15 = lshr i64 %_17, 1
  %7 = load i64, ptr %right, align 8, !noundef !2
  %8 = xor i64 %7, %_15
  store i64 %8, ptr %right, align 8
  %_31 = load i64, ptr %left, align 8, !noundef !2
  %_32 = load i64, ptr %right, align 8, !noundef !2
  %_30 = icmp slt i64 %_31, %_32
  br i1 %_30, label %bb3, label %bb4

bb4:                                              ; preds = %start
  %_34 = load i64, ptr %left, align 8, !noundef !2
  %_35 = load i64, ptr %right, align 8, !noundef !2
  %_33 = icmp eq i64 %_34, %_35
  br i1 %_33, label %bb5, label %bb6

bb3:                                              ; preds = %start
  store i8 -1, ptr %2, align 1
  br label %bb8

bb8:                                              ; preds = %bb7, %bb3
  %9 = load i8, ptr %2, align 1, !range !3, !noundef !2
  ret i8 %9

bb6:                                              ; preds = %bb4
  store i8 1, ptr %2, align 1
  br label %bb7

bb5:                                              ; preds = %bb4
  store i8 0, ptr %2, align 1
  br label %bb7

bb7:                                              ; preds = %bb6, %bb5
  br label %bb8
}

; probe1::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe15probe17h53e4a1424216b97dE() unnamed_addr #1 {
start:
; call core::f64::<impl f64>::total_cmp
  %_1 = call i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$9total_cmp17hf2bb722132daa76dE"(ptr align 8 @alloc6, ptr align 8 @alloc8), !range !3
  ret void
}

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{}
!3 = !{i8 -1, i8 2}
