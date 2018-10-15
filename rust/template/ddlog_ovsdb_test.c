#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#include "ddlog.h"

int main(int args, char *argv)
{
	datalog_example_ddlog_prog prog = datalog_example_run(1);
	if (prog == NULL) {
		fprintf(stderr, "failed to initialize DDlog program\n");
		return -1;
	};

	// Each line in the input stream is a JSON <table-updates> value.

	while(true) {
		char *lineptr = NULL;
		size_t n = 0;
		if (getline(&lineptr, &n, stdin) < 0) {
			// eof
			break;
		}
		//printf("updates: %s\n", lineptr);
		// start transaction
		if (datalog_example_transaction_start(prog) < 0) {
			fprintf(stderr, "failed to start transaction\n");
			return -1;
		};
		// apply updates
		if (datalog_example_apply_ovsdb_updates(prog, "", lineptr) < 0) {
			fprintf(stderr, "failed to apply updates (%s)\n", lineptr);
			return -1;
		};
		free(lineptr);
		// commit
		if (datalog_example_transaction_commit(prog) < 0) {
			fprintf(stderr, "failed to commit transaction\n");
			return -1;
		};
	}

	if (datalog_example_stop(prog) < 0) {
		fprintf(stderr, "failed to stop DDlog program\n");
		return -1;
	}
	return 0;
}
