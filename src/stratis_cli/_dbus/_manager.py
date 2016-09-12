# Copyright 2016 Red Hat, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
Manager interface.
"""

from ._dbus import Properties


class Manager(object):
    """
    Manager interface.
    """

    _INTERFACE_NAME = 'org.storage.stratis1.Manager'

    def __init__(self, dbus_object):
        """
        Initializer.

        :param dbus_object: the dbus object
        """
        self._dbus_object = dbus_object

    def CreatePool(self, pool_name, devices, redundancy):
        """
        Create a pool.

        :param str pool_name: the pool name
        :param devices: the component devices
        :type devices: sequence of str
        :param int redundancy: redundancy for this pool
        """
        return self._dbus_object.CreatePool(
           pool_name,
           devices,
           redundancy,
           dbus_interface=self._INTERFACE_NAME,
        )

    def DestroyPool(self, pool_name):
        """
        Destroy a pool.

        :param str pool_name: the name of the pool
        """
        return self._dbus_object.DestroyPool(
           pool_name,
           dbus_interface=self._INTERFACE_NAME
        )

    def GetCacheObjectPath(self, pool):
        """
        Get cache object path.

        :param str pool: the name of the pool
        """
        return self._dbus_object.GetCacheObjectPath(
           pool,
           dbus_interface=self._INTERFACE_NAME
        )

    def GetErrorCodes(self):
        """
        Get stratisd error codes.

        :rtype: Array of String * Int32 * String
        """
        return self._dbus_object.GetErrorCodes(
           dbus_interface=self._INTERFACE_NAME
        )

    def GetPoolObjectPath(self, pool_name):
        """
        Get the object path of a pool.

        :param str pool_name: the name of the pool
        """
        return self._dbus_object.GetPoolObjectPath(
           pool_name,
           dbus_interface=self._INTERFACE_NAME
        )

    def GetRaidLevels(self):
        """
        Get all designated RAID levels.

        :returns: list of RAID levels
        :rtype: Array of String * Int32 * String
        """
        return self._dbus_object.GetRaidLevels(
           dbus_interface=self._INTERFACE_NAME
        )

    def GetVolumeObjectPath(self, pool_name, volume_name):
        """
        Get the object path of volume ``volume_name`` in pool ``pool_name``.

        :param str pool_name: the pool name
        :param str volume_name: the volume name
        """
        return self._dbus_object.GetVolumeObjectPath(
           pool_name,
           volume_name,
           dbus_interface=self._INTERFACE_NAME
        )

    def ListPools(self):
        """
        List all pools.
        """
        return self._dbus_object.ListPools(dbus_interface=self._INTERFACE_NAME)

    @property
    def Version(self):
        """
        Stratisd Version getter.

        :rtype: String
        """
        return Properties(self._dbus_object).Get(
           self._INTERFACE_NAME,
           'Version'
        )

    @property
    def LogLevel(self):
        """
        Stratisd LogLevel getter.

        :rtype: String
        """
        return Properties(self._dbus_object).Get(
           self._INTERFACE_NAME,
           'LogLevel'
        )