#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_get_repo(int64_t port_);

void wire_set_repo(int64_t port_, struct wire_uint_8_list *path);

void wire_open_default_repo(int64_t port_);

void wire_get_default_repo(int64_t port_);

void wire_fetch_status(int64_t port_);

void wire_get_diff(int64_t port_);

void wire_get_status_items(int64_t port_);

void wire_set_status_select(int64_t port_, uintptr_t index);

void wire_get_index_items(int64_t port_);

void wire_index_add(int64_t port_);

void wire_commit(int64_t port_, struct wire_uint_8_list *msg);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_get_repo);
    dummy_var ^= ((int64_t) (void*) wire_set_repo);
    dummy_var ^= ((int64_t) (void*) wire_open_default_repo);
    dummy_var ^= ((int64_t) (void*) wire_get_default_repo);
    dummy_var ^= ((int64_t) (void*) wire_fetch_status);
    dummy_var ^= ((int64_t) (void*) wire_get_diff);
    dummy_var ^= ((int64_t) (void*) wire_get_status_items);
    dummy_var ^= ((int64_t) (void*) wire_set_status_select);
    dummy_var ^= ((int64_t) (void*) wire_get_index_items);
    dummy_var ^= ((int64_t) (void*) wire_index_add);
    dummy_var ^= ((int64_t) (void*) wire_commit);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
