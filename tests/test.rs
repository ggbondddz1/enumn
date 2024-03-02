# #!/usr/bin/python
# # -*- coding: UTF-8 -*-
# from commonlib.base_lib.mylog.mylog import log
# import pytest
# from src.Api_Lib.parameter_verify_lib import *
# from src.Api_Lib.RcdcApi.WEB.printer_web import Printerweb
# from src.TestCase.RcdcCase.WEB.printer_web.common_printer_web import printer_web_common_assert
# from src.TestCase.RcdcCase.WEB.printer_web.conftest import open_printer_testdata_SHARE
#
#
# class TestPrinterwebParameter(object):
#     @pytest.mark.unpublish
#     @pytest.mark.case_level_3
#     @pytest.mark.case_type_parameter
#     @pytest.mark.parametrize('idarr', base_uuid)
#     def test_delete_printer_idarr(self, idarr):
#         """
#         用例名称:删除打印机配置接口的idarr参数验证用例
#         接口名称:删除打印机配置
#         用例作者:
#         测试点:删除打印机配置接口的idarr参数验证用例
#         前置步骤:
#         执行步骤:
#         校验点:
#         """
#         log(">>>>>>测试[删除打印机配置]参数idarr用例>>>>>")
#         libs = Printerweb()
#         data = libs.delete_printer_data(idarr=idarr)
#         result = common_parameter_verify(libs.delete_printer, data=data)
#         print(result)
#         # if idarr in base_uuid:
#         #     printer_web_common_assert(result.json(), message='')
#         # else:
#         #     printer_web_common_assert(result.json(), message='')
#
#     @pytest.mark.bug_case
#     @pytest.mark.case_level_3
#     @pytest.mark.case_type_parameter
#     @pytest.mark.parametrize('id', base_uuid)
#     def test_detail_printer_id(self, id):
#         """
#         用例名称:查询打印机配置详情接口的id参数验证用例
#         接口名称:查询打印机配置详情
#         用例作者:
#         测试点:查询打印机配置详情接口的id参数验证用例
#         前置步骤:
#         执行步骤:
#         校验点:
#         """
#         log(">>>>>>测试[查询打印机配置详情]参数id用例>>>>>")
#         libs = Printerweb()
#         data = libs.detail_printer_data(id=id)
#         result = common_parameter_verify(libs.detail_printer, data=data)
#         print('ddddddddddddddd')
#         print(result)
#         # if id in base_uuid:
#         #     printer_web_common_assert(result.json(), message='')
#         # else:
#         #     printer_web_common_assert(result.json(), message='')
#
#
#     @pytest.mark.publish
#     @pytest.mark.case_level_3
#     @pytest.mark.case_type_parameter
#     @pytest.mark.parametrize('searchkeyword', base_name)
#     def test_list_printer_configname(self, searchkeyword):
#         """
#         用例名称:查询打印机配置接口的configname参数验证用例
#         接口名称:查询打印机配置
#         用例作者:
#         测试点:查询打印机配置接口的configname参数验证用例
#         前置步骤:
#         执行步骤:
#         校验点:
#         """
#         log(">>>>>>测试[查询打印机配置]参数configname用例>>>>>")
#         libs = Printerweb()
#         data = libs.list_printer_data(searchkeyword=searchkeyword)
#         result = common_parameter_verify(libs.list_printer, data=data)
#         print('ddddddddddddddd')
#         print(result.json())
#         if searchkeyword in base_name:
#             printer_web_common_assert(result.json(), message=None, status='SUCCESS')
#         else:
#             printer_web_common_assert(result.json(), message=None, status='SUCCESS')
#
#
#     # @pytest.mark.unpublish
#     # @pytest.mark.case_level_3
#     # @pytest.mark.case_type_parameter
#     # @pytest.mark.parametrize('printermodel', base_str + base_param+[str_129])
#     # #"该参数无需传参，用例废弃"
#     # def test_list_printer_printermodel(self, printermodel):
#     #     """
#     #     用例名称:查询打印机配置接口的printermodel参数验证用例
#     #     接口名称:查询打印机配置
#     #     用例作者:
#     #     测试点:查询打印机配置接口的printermodel参数验证用例
#     #     前置步骤:
#     #     执行步骤:
#     #     校验点:
#     #     """
#     #     log(">>>>>>测试[查询打印机配置]参数printermodel用例>>>>>")
#     #     libs = Printerweb()
#     #     data = libs.list_printer_data(printermodel=printermodel)
#     #     result = common_parameter_verify(libs.list_printer, data=data)
#     #     if printermodel in base_str:
#     #         printer_web_common_assert(result.json(), message='')
#     #     elif printermodel in base_param:
#     #         printer_web_common_assert(result.json(), message='')
#     #     elif printermodel in [str_129]:
#     #         printer_web_common_assert(result.json(), message='')
#     #     else:
#     #         printer_web_common_assert(result.json(), message='')
#     #
#     #
#     # @pytest.mark.unpublish
#     # @pytest.mark.case_level_3
#     # @pytest.mark.case_type_parameter
#     # @pytest.mark.parametrize('printerconnecttype', base_str + base_param)
#     # #"该参数无需传参，用例废弃"
#     # def test_list_printer_printerconnecttype(self, printerconnecttype):
#     #     """
#     #     用例名称:查询打印机配置接口的printerconnecttype参数验证用例
#     #     接口名称:查询打印机配置
#     #     用例作者:
#     #     测试点:查询打印机配置接口的printerconnecttype参数验证用例
#     #     前置步骤:
#     #     执行步骤:
#     #     校验点:
#     #     """
#     #     log(">>>>>>测试[查询打印机配置]参数printerconnecttype用例>>>>>")
#     #     libs = Printerweb()
#     #     data = libs.list_printer_data(printerconnecttype=printerconnecttype)
#     #     result = common_parameter_verify(libs.list_printer, data=data)
#     #     if printerconnecttype in base_str:
#     #         printer_web_common_assert(result.json(), message='')
#     #     elif printerconnecttype in base_param:
#     #         printer_web_common_assert(result.json(), message='')
#     #     else:
#     #         printer_web_common_assert(result.json(), message='')
#     #
#     # @pytest.mark.unpublish
#     # @pytest.mark.case_level_3
#     # @pytest.mark.case_type_parameter
#     # @pytest.mark.parametrize('createtime', base_str + base_param)
#     # #"该参数无需传参，用例废弃"
#     # def test_list_printer_createtime(self, createtime):
#     #     """
#     #     用例名称:查询打印机配置接口的createtime参数验证用例
#     #     接口名称:查询打印机配置
#     #     用例作者:
#     #     测试点:查询打印机配置接口的createtime参数验证用例
#     #     前置步骤:
#     #     执行步骤:
#     #     校验点:
#     #     """
#     #     log(">>>>>>测试[查询打印机配置]参数createtime用例>>>>>")
#     #     libs = Printerweb()
#     #     data = libs.list_printer_data(createtime=createtime)
#     #     result = common_parameter_verify(libs.list_printer, data=data)
#     #     if createtime in base_str:
#     #         printer_web_common_assert(result.json(), message='')
#     #     elif createtime in base_param:
#     #         printer_web_common_assert(result.json(), message='')
#     #     else:
#     #         printer_web_common_assert(result.json(), message='')
#     #
#
#     @pytest.mark.bug_case
#     @pytest.mark.case_level_3
#     @pytest.mark.case_type_parameter
#     @pytest.mark.parametrize('pageno', base_str + base_int)
#     def test_list_printer_pageno(self, pageno):
#         """
#         用例名称:查询打印机配置接口的pageno参数验证用例
#         接口名称:查询打印机配置
#         用例作者:
#         测试点:查询打印机配置接口的pageno参数验证用例
#         前置步骤:
#         执行步骤:
#         校验点:
#         """
#         log(">>>>>>测试[查询打印机配置]参数pageno用例>>>>>")
#         libs = Printerweb()
#         data = libs.list_printer_data(page=pageno)
#         result = common_parameter_verify(libs.list_printer, data=data)
#         assert  False
#         # if pageno in base_str:
#         #     printer_web_common_assert(result.json(), message='')
#         # elif pageno in base_int:
#         #     printer_web_common_assert(result.json(), message='')
#         # else:
#         #     printer_web_common_assert(result.json(), message='')
#
#     @pytest.mark.bug_case
#     @pytest.mark.case_level_3
#     @pytest.mark.case_type_parameter
#     @pytest.mark.parametrize('pagesize', base_str + base_int)
#     def test_list_printer_pagesize(self, pagesize):
#         """
#         用例名称:查询打印机配置接口的pagesize参数验证用例
#         接口名称:查询打印机配置
#         用例作者:
#         测试点:查询打印机配置接口的pagesize参数验证用例
#         前置步骤:
#         执行步骤:
#         校验点:
#         """
#         log(">>>>>>测试[查询打印机配置]参数pagesize用例>>>>>")
#         libs = Printerweb()
#         data = libs.list_printer_data(limit=pagesize)
#         result = common_parameter_verify(libs.list_printer, data=data)
#         #assert False
#         # if pagesize in base_str:
#         #     printer_web_common_assert(result.json(), message='')
#         # elif pagesize in base_int:
#         #     printer_web_common_assert(result.json(), message='')
#         # else:
#         #     printer_web_common_assert(result.json(), message='')
#
#
#     @pytest.mark.bug_case
#     @pytest.mark.case_level_3
#     @pytest.mark.case_type_parameter
#     @pytest.mark.parametrize('configname', base_name)
#     def test_edit_printer_configname(self,open_printer_testdata_SHARE, configname):
#         """
#         用例名称:编辑打印机配置接口的configname参数验证用例
#         接口名称:编辑打印机配置
#         用例作者:
#         测试点:编辑打印机配置接口的configname参数验证用例
#         前置步骤:
#         执行步骤:
#         校验点:
#         """
#         log(">>>>>>测试[编辑打印机配置]参数configname用例>>>>>")
#         libs = Printerweb()
#
#         data = libs.edit_printer_data(configname=configname,id=open_printer_testdata_SHARE)
#         result = common_parameter_verify(libs.edit_printer, data=data)
#         assert False
#         # if configname in base_name:
#         #     printer_web_common_assert(result.json(), message='')
#         # else:
#         #     printer_web_common_assert(result.json(), message='')
#
#
#     @pytest.mark.publish
#     @pytest.mark.case_level_3
#     @pytest.mark.case_type_parameter
#     @pytest.mark.parametrize('configdescription', base_ip)
#     def test_edit_printer_configdescription(self, open_printer_testdata_SHARE, configdescription):
#         """
#         用例名称:编辑打印机配置接口的configdescription参数验证用例
#         接口名称:编辑打印机配置
#         用例作者:
#         测试点:编辑打印机配置接口的configdescription参数验证用例
#         前置步骤:
#         执行步骤:
#         校验点:
#         """
#         log(">>>>>>测试[编辑打印机配置]参数configdescription用例>>>>>")
#         libs = Printerweb()
#         data = libs.edit_printer_data(configname='默认配置名', configdescription=configdescription,id=open_printer_testdata_SHARE)
#         result = common_parameter_verify(libs.edit_printer, data=data)
#         if configdescription in base_ip:
#             printer_web_common_assert(result.json(), message='操作成功', status='SUCCESS')
#         else:
#             printer_web_common_assert(result.json(), message='操作成功', status='SUCCESS')
#
#
#     @pytest.mark.bug_case
#     @pytest.mark.case_level_3
#     @pytest.mark.case_type_parameter
#     @pytest.mark.parametrize('id', base_uuid)
#     def test_edit_printer_id(self, id):
#         """
#         用例名称:编辑打印机配置接口的id参数验证用例
#         接口名称:编辑打印机配置
#         用例作者:
#         测试点:编辑打印机配置接口的id参数验证用例
#         前置步骤:
#         执行步骤:
#         校验点:
#         """
#         log(">>>>>>测试[编辑打印机配置]参数id用例>>>>>")
#         libs = Printerweb()
#         data = libs.edit_printer_data(id=id)
#         result = common_parameter_verify(libs.edit_printer, data=data)
#         # if id in base_uuid:
#         #     printer_web_common_assert(result.json(), message='')
#         # else:
#         #     printer_web_common_assert(result.json(), message='')
#
