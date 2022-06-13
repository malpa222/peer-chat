#!/bin/sh

diesel setup &&\
diesel migration run &&\
./user-service