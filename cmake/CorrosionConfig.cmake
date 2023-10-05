get_filename_component(PACKAGE_PREFIX_DIR "${CMAKE_CURRENT_LIST_DIR}" ABSOLUTE)

macro(set_and_check _var _file)
  set(${_var} "${_file}")
  if(NOT EXISTS "${_file}")
    message(FATAL_ERROR "File or directory ${_file} referenced by variable ${_var} does not exist !")
  endif()
endmacro()

macro(check_required_components _NAME)
  foreach(comp ${${_NAME}_FIND_COMPONENTS})
    if(NOT ${_NAME}_${comp}_FOUND)
      if(${_NAME}_FIND_REQUIRED_${comp})
        set(${_NAME}_FOUND FALSE)
      endif()
    endif()
  endforeach()
endmacro()


if (Corrosion_FOUND)
    return()
endif()

set(CORROSION_NATIVE_TOOLING_INSTALLED OFF)
if(CORROSION_NATIVE_TOOLING_INSTALLED AND NOT TARGET Corrosion::Generator)
    add_executable(Corrosion::Generator IMPORTED GLOBAL)
endif()

include(Corrosion)
