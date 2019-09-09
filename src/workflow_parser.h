#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  CTriggerAtom,
} CTrigger_Tag;

typedef struct {
  const char *_0;
} CTriggerAtom_Body;

typedef struct {
  CTrigger_Tag tag;
  union {
    CTriggerAtom_Body ctrigger_atom;
  };
} CTrigger;

typedef struct {
  CTrigger on;
  const char *name;
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
