// Created by yejq

#ifndef WUBIDICT_H
#define WUBIDICT_H 1
#ifdef __cplusplus
extern "C" {
#endif

typedef void * WubiForm;

#define WFB_QUERY    0
#define WFB_LOAD     1
#define WFB_STORE    2
#define WFB_RECALL   3
#define WFB_HPREV    4
#define WFB_HNEXT    5

#ifdef _MSC_VER
  #define Q_DECL_EXPORT0 __declspec(dllexport)
  #define Q_DECL_EXPORT1
#else
  #define Q_DECL_EXPORT0
  #define Q_DECL_EXPORT1 __attribute__((visibility("default")))
#endif

typedef int (* WformButtonCb)(WubiForm form,
	const char * utf8p, unsigned int utf8l);

WubiForm Q_DECL_EXPORT0 wform_init_lib(void) Q_DECL_EXPORT1;

int Q_DECL_EXPORT0 wform_looping(WubiForm form) Q_DECL_EXPORT1;

int Q_DECL_EXPORT0 wform_push_result(WubiForm form,
	const char * utf8p, unsigned int utf8l) Q_DECL_EXPORT1;

int Q_DECL_EXPORT0 wform_regiter_fun(WubiForm form,
	int which, WformButtonCb fcb) Q_DECL_EXPORT1;

#ifdef __cplusplus
}
#endif
#endif
