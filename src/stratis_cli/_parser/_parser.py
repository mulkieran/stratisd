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
Top level parser for Stratis CLI.
"""


import argparse

from .._actions import StratisActions
from .._actions import TopActions

from .._version import __version__

from ._logical import build_logical_parser
from ._physical import build_physical_parser


def build_stratisd_parser(parser):
    """
    Generates the parser appropriate for obtaining information about stratisd.

    :param ArgumentParser parser: a parser
    :returns: a completed parser for obtaining information about stratisd
    :rtype: ArgumentParser
    """
    group = parser.add_mutually_exclusive_group(required=True)

    group.add_argument(
      '--log-level',
      action='store_true',
      default=False,
      help='log level of stratisd daemon'
    )
    group.add_argument(
      '--redundancy',
      action='store_true',
      default=False,
      help='redundancy designations understood by stratisd daemon'
    )
    group.add_argument(
       '--version',
       action='store_true',
       default=False,
       help='version of stratisd daemon'
    )

    parser.set_defaults(func=StratisActions.dispatch)
    return parser


def build_list_parser(parser):
    """
    Generate the parser appropriate for displaying information.

    :param ArgumentParser parser: a parser
    :returns: a completed parser for listing pools
    :rtype: ArgumentParser
    """
    parser.set_defaults(func=TopActions.list_pools)
    return parser


def build_create_parser(parser):
    """
    Generates the parser appropriate for creating a pool.

    :param ArgumentParser parser: a parser
    :returns: a completed parser for creating a pool
    :rtype: ArgumentParser
    """
    parser.add_argument(
       'name',
       action='store',
       help='name to assign to pool'
    )
    parser.add_argument(
       'device',
       help='make device D a member of this pool',
       metavar='D',
       nargs='+'
    )
    parser.add_argument(
       '--redundancy',
       action='store',
       choices=['none'],
       default='none',
       help="redundancy selection for this pool"
    )
    parser.set_defaults(func=TopActions.create_pool)
    return parser


def build_destroy_parser(parser):
    """
    Generates the parser appropriate for destroying a pool.

    :param ArgumentParser parser: a parser
    :returns: a completed parser for destroying a pool
    :rtype: ArgumentParser
    """
    parser.add_argument(
       'name',
       action='store',
       help='name of pool'
    )
    parser.set_defaults(func=TopActions.destroy_pool)
    return parser


def build_rename_parser(parser):
    """
    Generates the parser appropriate for renaming a pool.

    :param ArgumentParser parser: a parser
    :returns: a completed parser for renaming a pool
    :rtype: ArgumentParser
    """
    parser.add_argument(
       'current',
       action='store',
       help='current name of pool'
    )
    parser.add_argument('new', action='store', help='desired name')
    parser.set_defaults(func=TopActions.rename_pool)
    return parser


_SUBPARSER_TABLE = {
   'blockdev' : build_physical_parser,
   'create' : build_create_parser,
   'destroy' : build_destroy_parser,
   'list' : build_list_parser,
   'filesystem' : build_logical_parser,
   'stratisd' : build_stratisd_parser,
   'rename' : build_rename_parser
}


def gen_parser():
    """
    Make the parser.

    :returns: a fully constructed parser for command-line arguments
    :rtype: ArgumentParser
    """
    parser = argparse.ArgumentParser(
       description="Stratis Storage Manager",
       prog='stratis'
    )

    parser.add_argument(
       '--version',
       action='version',
       version=__version__
    )

    subparsers = \
       parser.add_subparsers(dest='subparser_name', title='subcommands')

    subparser_table = dict()

    subparser_table['blockdev'] = \
       subparsers.add_parser(
          'blockdev',
          description="Administer Blockdev Aspects of Specified Pool"
       )
    _SUBPARSER_TABLE['blockdev'](subparser_table['blockdev'])

    subparser_table['create'] = \
       subparsers.add_parser('create', description="Create New Stratis Pool")
    _SUBPARSER_TABLE['create'](subparser_table['create'])

    subparser_table['destroy'] = \
       subparsers.add_parser(
          'destroy',
          description="Destroy Existing Stratis Pool"
    )
    _SUBPARSER_TABLE['destroy'](subparser_table['destroy'])

    subparser_table['filesystem'] = \
       subparsers.add_parser(
          'filesystem',
          description="Administer Filesystems Aspects of Specified Pool"
       )
    _SUBPARSER_TABLE['filesystem'](subparser_table['filesystem'])

    subparser_table['list'] = \
       subparsers.add_parser('list', description="List Stratis Pools")
    _SUBPARSER_TABLE['list'](subparser_table['list'])

    subparser_table['stratisd'] = \
       subparsers.add_parser(
          'stratisd',
          description="Information about Stratisd"
    )
    _SUBPARSER_TABLE['stratisd'](subparser_table['stratisd'])

    subparser_table['rename'] = \
       subparsers.add_parser('rename', description="Rename a Pool")
    _SUBPARSER_TABLE['rename'](subparser_table['rename'])

    return parser