#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#include "log.h"

typedef unsigned int log_type;

typedef unsigned int xcm_tp_cnt;

typedef unsigned int xcm_socket_type;

typedef struct xcm_tp_ops {
  int (*init)(struct xcm_socket*, struct xcm_socket*);
  int (*connect)(struct xcm_socket*, const char*);
  int (*server)(struct xcm_socket*, const char*);
  void (*close)(struct xcm_socket*);
  void (*cleanup)(struct xcm_socket*);
  int (*accept)(struct xcm_socket*, struct xcm_socket*);
  int (*send)(struct xcm_socket*, const void*, size_t);
  int (*receive)(struct xcm_socket*, void*, size_t);
  void (*update)(struct xcm_socket*);
  int (*finish)(struct xcm_socket*);
  const char *(*get_transport)(struct xcm_socket*);
  const char *(*get_remote_addr)(struct xcm_socket*, bool);
  const char *(*get_local_addr)(struct xcm_socket*, bool);
  int (*set_local_addr)(struct xcm_socket*, const char*);
  size_t (*max_msg)(struct xcm_socket*);
  int64_t (*get_cnt)(struct xcm_socket*, xcm_tp_cnt);
  void (*enable_ctl)(struct xcm_socket*);
  void (*attr_populate)(struct xcm_socket*, attr_tree*);
  size_t (*priv_size)(xcm_socket_type);
} xcm_tp_ops;

typedef struct xcm_tp_proto {
  char name[33];
  const struct xcm_tp_ops *ops;
} xcm_tp_proto;

typedef struct xcm_socket {
  const struct xcm_tp_proto *proto;
  xcm_socket_type type_0;
  int64_t sock_id;
  bool auto_enable_ctl;
  bool auto_update;
  bool is_blocking;
  xpoll *xpoll;
  int condition;
  ctl *ctl;
  uint64_t skipped_ctl_calls;
} xcm_socket;

typedef unsigned int attr_pcomp_type;

typedef union C2RustUnnamed {
  char *key;
  size_t index;
} C2RustUnnamed;

typedef struct attr_pcomp {
  attr_pcomp_type type_0;
  union C2RustUnnamed c2rust_unnamed;
} attr_pcomp;

typedef struct attr_path {
  struct attr_pcomp *comps[64];
  size_t num_comps;
} attr_path;

#define attr_pcomp_type_index 1

#define attr_pcomp_type_key 0

#define log_type_debug 0

#define log_type_error 1

#define xcm_socket_type_conn 0

#define xcm_socket_type_server 1

#define xcm_tp_cnt_from_app_bytes 1

#define xcm_tp_cnt_from_app_msgs 5

#define xcm_tp_cnt_from_lower_bytes 3

#define xcm_tp_cnt_from_lower_msgs 7

#define xcm_tp_cnt_to_app_bytes 0

#define xcm_tp_cnt_to_app_msgs 4

#define xcm_tp_cnt_to_lower_bytes 2

#define xcm_tp_cnt_to_lower_msgs 6

extern void __log_event(log_type type_0,
                        const char *file,
                        int line,
                        const char *function,
                        struct xcm_socket *s,
                        const char *format,
                        ...);

extern void abort(void);

void attr_path_destroy(struct attr_path *path);

bool attr_path_equal(const struct attr_path *path_a, const struct attr_path *path_b);

bool attr_path_equal_str(const struct attr_path *path, const char *path_str, bool root);

const struct attr_pcomp *attr_path_get_comp(const struct attr_path *path, size_t comp_num);

bool attr_path_is_valid_key(const char *key);

size_t attr_path_len(const struct attr_path *path, bool root);

size_t attr_path_num_comps(const struct attr_path *path);

struct attr_path *attr_path_parse(const char *path_str, bool root);

char *attr_path_to_str(const struct attr_path *path, bool root);

size_t attr_pcomp_get_index(const struct attr_pcomp *pcomp);

const char *attr_pcomp_get_key(const struct attr_pcomp *pcomp);

attr_pcomp_type attr_pcomp_get_type(const struct attr_pcomp *pcomp);

bool attr_pcomp_is_index(const struct attr_pcomp *pcomp);

bool attr_pcomp_is_key(const struct attr_pcomp *pcomp);

extern void log_console_conf(bool enabled);

extern bool log_is_enabled(log_type type_0);

extern int snprintf(char*, unsigned long, const char*, ...);

extern int strcmp(const char*, const char*);

extern unsigned long strlen(const char*);

extern long strtol(const char*, char**, int);

extern void *ut_calloc(size_t size);

extern void ut_free(void *ptr);

extern void *ut_malloc(size_t size);

extern char *ut_strdup(const char *str);
