#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct HashMap_CString__CJob HashMap_CString__CJob;

typedef struct Option_CStringList Option_CStringList;

typedef struct {
  Option_CStringList branches;
  Option_CStringList tags;
} CTriggerPushInner;

typedef struct {
  CString cron;
  Option_CStringList branches;
  Option_CStringList tags;
} CTriggerScheduleInner;

typedef enum {
  CTriggerAtom,
  CTriggerList,
  CTriggerPush,
  CTriggerSchedule,
} CTrigger_Tag;

typedef struct {
  CString _0;
} CTriggerAtom_Body;

typedef struct {
  CVec<CString> _0;
} CTriggerList_Body;

typedef struct {
  CTriggerPushInner push;
} CTriggerPush_Body;

typedef struct {
  CVec<CTriggerScheduleInner> schedule;
} CTriggerSchedule_Body;

typedef struct {
  CTrigger_Tag tag;
  union {
    CTriggerAtom_Body ctrigger_atom;
    CTriggerList_Body ctrigger_list;
    CTriggerPush_Body ctrigger_push;
    CTriggerSchedule_Body ctrigger_schedule;
  };
} CTrigger;

typedef struct {
  CTrigger on;
  CString name;
  HashMap_CString__CJob jobs;
} CWorkflow;

typedef enum {
  Ok,
  Err,
} CResult_Tag;

typedef struct {
  CWorkflow _0;
} Ok_Body;

typedef struct {
  const char *_0;
} Err_Body;

typedef struct {
  CResult_Tag tag;
  union {
    Ok_Body ok;
    Err_Body err;
  };
} CResult;

const char *err_from_result(CResult r);

bool is_result_ok(CResult r);

CResult parse(char *c_str_p);

CWorkflow workflow_from_result(CResult r);
