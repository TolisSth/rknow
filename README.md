# RKnow: A 0 dependencies profiler written in Rust for linux systems
Warning! This is still a WIP so do not use in production. 
This is part of an HPC research project, in measuring the power/energy efficiency of an ARM cluster vs an x86 system.

Most of those measurements rely on the /proc filesystem in linux that gives access to lower-level information.

It is meant to be used as a daemon when running HPC experiments in order to assess the system under test.

The following things will be measured(some are already, some not):
- RAM usage (IOops,memory usage,)
- CPU usage (temperature, per core utilisation %, total utilisation %, Clock Speed)
- Network Related information (packets received, transmitted etc. etc.)
- Disk Usage (needed for HPC experiments that require large datasets)

If necessary, more things will be measured in the future.

