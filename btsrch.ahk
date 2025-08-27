#Requires AutoHotkey v2.0

SetWorkingDir A_ScriptDir

!Space::
{
    Run(".\target\release\btsrch.exe")
    return
}