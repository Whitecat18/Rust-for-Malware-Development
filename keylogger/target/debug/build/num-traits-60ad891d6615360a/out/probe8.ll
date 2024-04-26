; ModuleID = 'probe8.50f98d2ae4e4492f-cgu.0'
source_filename = "probe8.50f98d2ae4e4492f-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; core::f64::<impl f64>::to_ne_bytes
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h8d30d9f16102efdfE"(double %self) unnamed_addr #0 {
start:
  %_3 = alloca double, align 8
  %0 = alloca [8 x i8], align 1
  store double %self, ptr %_3, align 8
  %rt = load double, ptr %_3, align 8, !noundef !2
  %self1 = bitcast double %rt to i64
  store i64 %self1, ptr %0, align 1
  %1 = load i64, ptr %0, align 1
  ret i64 %1
}

; probe8::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe85probe17h261546016dced920E() unnamed_addr #1 {
start:
  %0 = alloca i64, align 8
  %_1 = alloca [8 x i8], align 1
; call core::f64::<impl f64>::to_ne_bytes
  %1 = call i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h8d30d9f16102efdfE"(double 3.140000e+00)
  store i64 %1, ptr %0, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %_1, ptr align 8 %0, i64 8, i1 false)
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #2

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{}
