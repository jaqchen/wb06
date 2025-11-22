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
#define WFB_NEXT     3

typedef int (* WformButtonCb)(WubiForm form,
	const char * utf8p, unsigned int utf8l);

WubiForm wform_init_lib(void)
	__attribute__((visibility("default")));

int wform_looping(WubiForm form)
	__attribute__((visibility("default")));

int wform_push_result(WubiForm form,
	const char * utf8p, unsigned int utf8l)
	__attribute__((visibility("default")));

int wform_regiter_fun(WubiForm form, int which, WformButtonCb fcb)
	__attribute__((visibility("default")));

#ifdef __cplusplus
}
#endif
#endif
